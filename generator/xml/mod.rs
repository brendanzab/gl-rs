//! High level wrapper for libxml2's SAX parser

use std::cast::transmute;
use std::comm::Chan;
use std::hashmap::*;
use std::libc::{c_int, c_void};
use std::ptr::null;
use std::str::raw::{from_buf, from_buf_len};

use self::error::ErrorData;

pub mod error;
pub mod ffi;

#[deriving(Eq, Clone)]
pub enum Msg {
    StartElement(~str, Attributes),
    EndElement(~str),
    Characters(~str),
}

impl ToStr for Msg {
    fn to_str(&self) -> ~str {
        match *self {
            StartElement(ref name, ref atts) => fmt!("<%s%s>", *name, atts.to_str()),
            EndElement(ref name) => fmt!("</%s>", *name),
            Characters(ref chrs) => chrs.clone(),
        }
    }
}

pub struct Attributes(HashMap<~str, ~str>);

impl ToStr for Attributes {
    fn to_str(&self) -> ~str {
        do self.iter().transform |(k, v)| {
            fmt!(" %s=\"%s\"", *k, *v)
        }.to_owned_vec().concat()
    }
}

pub type ParseResult = Result<Msg, ErrorData>;

pub fn parse(src: &str, chan: Chan<ParseResult>) {
    unsafe {
        do src.as_c_str |c_str| {
            ffi::xmlSAXUserParseMemory(&new_sax_handler(),
                                       transmute(&chan),
                                       c_str, src.len() as c_int);
            ffi::xmlCleanupParser();
        }
    }
}

unsafe fn chan_from_ptr(ctx: *c_void) -> &Chan<ParseResult> {
    transmute(ctx)
}

extern "C" fn start_element(ctx: *c_void, name: *ffi::xmlChar, atts: **ffi::xmlChar) {
    unsafe {
        let mut map = Attributes(HashMap::new());
        let mut ptr = atts;
        while !ptr.is_null() && !(*ptr).is_null() {
            map.insert(from_buf(*ptr),
                       from_buf(*(ptr + 1)));
            ptr = ptr + 2;
        }

        chan_from_ptr(ctx).send(
            Ok(StartElement(from_buf(name), map))
        );
    }
}

extern "C" fn end_element(ctx: *c_void, name: *ffi::xmlChar) {
    unsafe {
        chan_from_ptr(ctx).send(
            Ok(EndElement(from_buf(name)))
        );
    }
}

extern "C" fn characters(ctx: *c_void, ch: *ffi::xmlChar, len: c_int) {
    unsafe {
        chan_from_ptr(ctx).send(
            Ok(Characters(from_buf_len(ch, len as uint)))
        );
    }
}

extern "C" fn serror(ctx: *c_void, error: *ffi::xmlError) {
    unsafe {
        do ErrorData::from_ptr(error).map |err| {
            chan_from_ptr(ctx).send(Err(err.clone()));
        };
    }
}

fn new_sax_handler() -> ffi::xmlSAXHandler {
    ffi::xmlSAXHandler {
        internalSubset:         null(),
        isStandalone:           null(),
        hasInternalSubset:      null(),
        hasExternalSubset:      null(),
        resolveEntity:          null(),
        getEntity:              null(),
        entityDecl:             null(),
        notationDecl:           null(),
        attributeDecl:          null(),
        elementDecl:            null(),
        unparsedEntityDecl:     null(),
        setDocumentLocator:     null(),
        startDocument:          null(),
        endDocument:            null(),
        startElement:           start_element,
        endElement:             end_element,
        reference:              null(),
        characters:             characters,
        ignorableWhitespace:    null(),
        processingInstruction:  null(),
        comment:                null(),
        warning:                null(),
        error:                  null(),
        fatalError:             null(),
        getParameterEntity:     null(),
        cdataBlock:             null(),
        externalSubset:         null(),
        initialized:            ffi::XML_SAX2_MAGIC,
        _private:               null(),
        startElementNs:         null(),
        endElementNs:           null(),
        serror:                 serror,
    }
}
