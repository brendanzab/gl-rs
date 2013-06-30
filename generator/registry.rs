use std::cast::transmute;
use std::io::{Create, file_reader, file_writer};
use std::libc::{c_int, c_void, size_t};
use std::local_data::{local_data_set, local_data_get};
use std::path::Path;
use std::ptr::null;
use std::str::from_bytes;
use std::str::raw::{from_buf, from_buf_len};
use std::vec::raw::from_buf_raw;
use super::xml;

priv fn curl_writer_tls_key(_: @@Writer) {}

trait CurlWriter {
    pub fn download_from(&self, url: &str);
}

impl CurlWriter for @Writer {
    pub fn download_from(&self, url: &str) {
        use rust_curl::curl::*;

        let curl = Curl::new();
        do url.as_c_str |c_str| { curl.easy_setopt(opt::URL, c_str); }

        unsafe { local_data_set(curl_writer_tls_key, @*self); }
        curl.easy_setopt(opt::WRITEFUNCTION, write_fn);

        match curl.easy_perform() {
            code::CURLE_OK => (),
            e => fail!(~"Download error: " + easy_strerror(e)),
        }
    }
}

priv extern "C" fn write_fn(data: *u8, size: size_t, nmemb: size_t, _: *c_void) -> size_t {
    let len = size * nmemb;
    unsafe {
        let new_data = from_buf_raw(data, len as uint);
        local_data_get(curl_writer_tls_key)
            .expect("Could not find writer.")
            .write(new_data);
    }
    len
}

pub fn downoad_src(path: Path, url: &str, reload: bool) -> ~[u8] {
    match file_reader(&path) {
        Ok(r) if !reload => r,
        _ => {
            let file = file_writer(&path, [Create]).get();
            file.download_from(url);
            file_reader(&path).get()
        }
    }.read_whole_stream()
}

pub fn parse_xml(data: &[u8]) -> Registry {
    let sax_handler = &xml::xmlSAXHandler {
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
        initialized:            xml::XML_SAX2_MAGIC,
        _private:               null(),
        startElementNs:         null(),
        endElementNs:           null(),
        serror:                 serror,
    };

    let registry = Registry {
        enums: ~[],
        commands: ~[],
    };
    unsafe {
        xml::parse_str(sax_handler, &registry, from_bytes(data));
    }
    registry
}

pub struct Registry {
    enums: ~[Enums],
    commands: ~[Commands]
}

pub struct Enums {
    namespace: ~str,
    etype: ~str,
    enums: ~[Enum],
}

pub struct Enum {
    name: ~str,
    value: ~str,
}

pub struct Commands {
    namespace: ~str,
    commands: ~[Command],
}

pub struct Command {
    name: ~str,
    rtype: Option<~str>,
    params: ~[Param]
}

pub struct Param {
    ptype: ~str,
    name: ~str,
}

impl Registry {
    pub unsafe fn from_c_void(ptr: *c_void) -> &mut Registry {
        transmute(ptr)
    }
}

priv extern "C" fn start_element(_ctx: *c_void, name: *xml::xmlChar, atts: **xml::xmlChar) {
    println("start_element");
    unsafe {
        println(fmt!("    name: %s", from_buf(name)));
    }
    println(fmt!("    attribs: %?", xml::build_attrib_vec(atts)));
}

priv extern "C" fn end_element(_ctx: *c_void, name: *xml::xmlChar) {
    unsafe { println(fmt!("end_element: %s", from_buf(name))); }
}

priv extern "C" fn characters(_ctx: *c_void, ch: *xml::xmlChar, len: c_int) {
    unsafe { println(fmt!("characters: %s", from_buf_len(ch, len as uint))); }
}

priv extern "C" fn serror(_ctx: *c_void, error: *xml::xmlError) {
    do unsafe { xml::ErrorData::from_ptr(error) }.map |err| {
        println(fmt!("%s", err.to_str()));
    };
}
