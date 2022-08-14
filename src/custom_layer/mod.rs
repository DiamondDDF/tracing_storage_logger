mod visitor;
use std::cell::Cell;
use std::collections::BTreeMap;
use std::path::PathBuf;
use chrono::{Local};
use chrono_tz::Tz;
use tracing_subscriber::registry::{Scope, SpanRef};
use tracing_subscriber::{Layer, registry::Extensions};
use visitor::*;
use file_rotate::{FileRotate, ContentLimit, suffix::AppendCount, compression::Compression};
use std::{io::Write};
use tracing::Subscriber;
use tracing_subscriber::registry::LookupSpan;
use file_rotate::{suffix::{AppendTimestamp, FileLimit}};
use string_builder::Builder;
use chrono_tz::US::Eastern;

// need a way to set the root path. A const doesn't work because I want to reuse this crate.
// I could store the path in the logger, but how do I retrieve it?
// If I pass it in every time, that's going to be a pain
// Can I store it in the layer?
// cutom layer needs to be passed as an empty struct, I want to be able to have a stack of spans.
pub struct CustomLayer{
    pub path: PathBuf,
    pub limit: ContentLimit
}
//     pub file_logger: FileRotate<T>
// }

// impl CustomLayer<T> {
//     fn new<T>(path: &str) -> Self {
//         let logger = FileRotate::new(
//             "any path",//log_path.clone(),
//             AppendTimestamp::default(FileLimit::MaxFiles(2)),
//             ContentLimit::Bytes(1),
//             Compression::None,
//             #[cfg(unix)]
//             None,
//         );
//     }
// }
impl<S> Layer<S> for CustomLayer
where
    S: Subscriber,
    S: for<'lookup> LookupSpan<'lookup>,
{
    // In this function, when a span gets created, 
    // 1. I get the span by id from the context, 
    // 2. I create a new serializer/recorder called a Visitor and give it a reference to my "fields" dictionary.
    // 3. I pass my Vistor(recorder) to attributes.record which calls the necessary functions on Visitor to serialize and store things.
    // 4. I then package it in a "CustomFieldStorage" which is just a tuple struct.
    // 5. I then stash it in the extensions field of the current span
    /// This doesn't seem necessary for our application. I commented out the part where it actually records.
    fn on_new_span(
        &self,
        // Attributes, I would think include things like keyword items that I give the span
        attrs: &tracing::span::Attributes<'_>,
        // Id of the span in case I need to look up other things about it.
        id: &tracing::span::Id,
        // Context in this case, I'm a layer in a subscriber, the thing creating the new span is the
        // tracing crate itself, it's going to send me... the time? I already have the span.
        // Maybe this is where there are dictionaries by span ID
        ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        let span = ctx.span(id).unwrap();
        // create new fields dictionary, pass it to a new JSONVisitor, which puts things into the fields
        // when I type info!("some messages") for example.
        let mut fields = BTreeMap::new();
        // visitor records specific data types in specific ways.
        let mut visitor = JsonVisitor(&mut fields);
        // So I have my visitor with a hashmap to store things, now I have
        // attributes send all its data to the thing I made.
        //attrs.record(&mut visitor);
        // CustomFieldStorage is another tuple stuct that stores a dictionary at .0 
        let storage = CustomFieldStorage(fields);
        // extensions belongs to the span and is extra storage.
        let mut extensions = span.extensions_mut();
        extensions.insert(storage);
    }
    /// on_record is for when I call .record to add attributes to the span itself.
    /// I don't foresee needing this but it was a part of the example code.
    /// In fact I commented out the part where it actually records so we don't take up too much space.
    fn on_record(
        &self,
        id: &tracing::span::Id,
        values: &tracing::span::Record<'_>,
        ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        // Get the span whose data is being recorded
        let span = ctx.span(id).unwrap();
        // Get a mutable reference to the data we created in new_span
        let mut extensions_mut = span.extensions_mut();
        let custom_field_storage: &mut CustomFieldStorage =
            extensions_mut.get_mut::<CustomFieldStorage>().unwrap();
        let json_data: &mut BTreeMap<String, serde_json::Value> = &mut custom_field_storage.0;

        // And add to using our old friend the visitor!
        let mut visitor = JsonVisitor(json_data);
        //values.record(&mut visitor);
    }

    fn on_event(&self, event: &tracing::Event<'_>, ctx: tracing_subscriber::layer::Context<'_, S>) {
        // Event scope should include all outer spans if I'm not mistaken. Notice it's in a variable
        // called Context. This is a general term in computer science. Your name and id at the bank is a
        // context for making a transaction.
       
        // create a new dictionary, pass it to the visitor which know how to put things in the dictionary, 
        // pass that to the event.record, which calls methods on the visitor. They should call it a recorder!
        let mut fields = BTreeMap::new();
        let mut visitor = JsonVisitor(&mut fields);

        // Record the event. What will we do with this data later? I thought I was just about to record it?
        event.record(&mut visitor);
        // If scope is none, I just don't want to print the scope in
        // the head of the message
        // If the scope is missing, 
        // 1. Span Header can be an empty space, exclude \n or something.
        // 2. zero indent
        // 3. file_path cannot be gotten from current span, must panic.
        let mut current_span: Option<&SpanRef<'_, S>> = None;
        let mut spans =  Vec::<SpanRef<'_, S>>::new();
        let mut indent = "".to_string();
        let mut path_header = Builder::default();
        if ctx.event_scope(event).is_some() {
            spans = ctx.event_scope(event).unwrap().collect::<Vec<SpanRef<'_, S>>>();
            current_span = Some(spans.first().unwrap());
            indent = "  ".to_string().repeat(spans.len());
            for name in spans.iter().rev() {
                path_header.append(format!("{}() => ", name.name()));
            }

        };
        
        // going to indent by span, perhaps that will be more readable.
        let mut file_path = self.path.clone();
        if let Some(file_name) = fields.get("path"){
            file_path.push(file_name.as_str().unwrap())
        }
        else {
            if(current_span.is_none()) {
                panic!("Tried to log something. Not within a span, and did not specifiy a 'path'");
            }
            file_path.push(current_span.unwrap().name());
        }
        let mut message = Builder::default();
        let now = Local::now().with_timezone(&Eastern);
        message.append(format!("{}{}\n",indent, now.format("%Y-%m-%d %H:%M:%S")));
        message.append(path_header.string().unwrap());
        message.append(format!("{}", indent));
        
        message.append("\n");
        message.append(format!("{}{}: \n", indent, current_span.unwrap().metadata().level()));
        for (name, value) in &fields {
            message.append(format!("{}  {}: {}\n", indent, name, value));
        }
        message.append("\n");
        
        let output = serde_json::json!({
            "target": event.metadata().target(),
            "name": event.metadata().name(),
            "level": format!("{:?}", event.metadata().level()),
            "fields": fields,
        });
        let mut log = FileRotate::new(
            file_path,
            AppendTimestamp::default(FileLimit::MaxFiles(2)),
            self.limit.clone(),
            Compression::None,
            #[cfg(unix)]
            None,
        );
        let _ = write!(log, "{}", message.string().unwrap());
        //println!("{}{}", serde_json::to_string_pretty(&output).unwrap());
    }
  
    
}
impl CustomLayer {
    pub fn get_span_path(){

    }
}

