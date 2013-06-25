#[link(name = "glgen",
       vers = "0.1")];
#[comment = "OpenGL function loader generator."];

// Requires libxml2

// This will be used to generate the loader from the [registry xml files]
// (https://cvs.khronos.org/svn/repos/ogl/trunk/doc/registry/public/api/)

use std::cast::transmute;
use std::libc::*;
use std::ptr::null;
use std::str::raw::{from_buf, from_buf_len, from_c_str};

pub mod libxml2 {
    use std::libc::*;

    pub type xmlChar = c_uchar;

    // These can be found in libxml/parser.h

    pub static XML_SAX2_MAGIC: c_uint = 0xDEEDBEAF;

    pub type internalSubsetSAXFunc          = *u8;
    pub type isStandaloneSAXFunc            = *u8;
    pub type hasInternalSubsetSAXFunc       = *u8;
    pub type hasExternalSubsetSAXFunc       = *u8;
    pub type resolveEntitySAXFunc           = *u8;
    pub type getEntitySAXFunc               = *u8;
    pub type entityDeclSAXFunc              = *u8;
    pub type notationDeclSAXFunc            = *u8;
    pub type attributeDeclSAXFunc           = *u8;
    pub type elementDeclSAXFunc             = *u8;
    pub type unparsedEntityDeclSAXFunc      = *u8;
    pub type setDocumentLocatorSAXFunc      = *u8;
    pub type startDocumentSAXFunc           = *u8;
    pub type endDocumentSAXFunc             = *u8;
    pub type startElementSAXFunc            = *u8;  // typedef void (*startElementSAXFunc) (void *ctx, const xmlChar *name, const xmlChar **atts);
    pub type endElementSAXFunc              = *u8;  // typedef void (*endElementSAXFunc) (void *ctx, const xmlChar *name);
    pub type referenceSAXFunc               = *u8;  // typedef void (*referenceSAXFunc) (void *ctx, const xmlChar *name);
    pub type charactersSAXFunc              = *u8;  // typedef void (*charactersSAXFunc) (void *ctx, const xmlChar *ch, int len);
    pub type ignorableWhitespaceSAXFunc     = *u8;  // typedef void (*ignorableWhitespaceSAXFunc) (void *ctx, const xmlChar *ch, int len);
    pub type processingInstructionSAXFunc   = *u8;  // typedef void (*processingInstructionSAXFunc) (void *ctx, const xmlChar *target, const xmlChar *data);
    pub type commentSAXFunc                 = *u8;  // typedef void (*commentSAXFunc) (void *ctx, const xmlChar *value);
    pub type warningSAXFunc                 = *u8;  // typedef void (XMLCDECL *warningSAXFunc) (void *ctx, const char *msg, ...) LIBXML_ATTR_FORMAT(2,3);
    pub type errorSAXFunc                   = *u8;  // typedef void (XMLCDECL *errorSAXFunc) (void *ctx, const char *msg, ...) LIBXML_ATTR_FORMAT(2,3);
    pub type fatalErrorSAXFunc              = *u8;  // typedef void (XMLCDECL *fatalErrorSAXFunc) (void *ctx, const char *msg, ...) LIBXML_ATTR_FORMAT(2,3);
    pub type getParameterEntitySAXFunc      = *u8;
    pub type cdataBlockSAXFunc              = *u8;
    pub type externalSubsetSAXFunc          = *u8;
    pub type startElementNsSAX2Func         = *u8;  // typedef void (*startElementNsSAX2Func) (void *ctx, const xmlChar *localname, const xmlChar *prefix, const xmlChar *URI, int nb_namespaces, const xmlChar **namespaces, int nb_attributes, int nb_defaulted, const xmlChar **attributes);
    pub type endElementNsSAX2Func           = *u8;  // typedef void (*endElementNsSAX2Func)   (void *ctx, const xmlChar *localname, const xmlChar *prefix, const xmlChar *URI);
    pub type xmlStructuredErrorFunc         = *u8;

    pub struct xmlSAXHandler {
        internalSubset:         internalSubsetSAXFunc,
        isStandalone:           isStandaloneSAXFunc,
        hasInternalSubset:      hasInternalSubsetSAXFunc,
        hasExternalSubset:      hasExternalSubsetSAXFunc,
        resolveEntity:          resolveEntitySAXFunc,
        getEntity:              getEntitySAXFunc,
        entityDecl:             entityDeclSAXFunc,
        notationDecl:           notationDeclSAXFunc,
        attributeDecl:          attributeDeclSAXFunc,
        elementDecl:            elementDeclSAXFunc,
        unparsedEntityDecl:     unparsedEntityDeclSAXFunc,
        setDocumentLocator:     setDocumentLocatorSAXFunc,
        startDocument:          startDocumentSAXFunc,
        endDocument:            endDocumentSAXFunc,
        startElement:           startElementSAXFunc,
        endElement:             endElementSAXFunc,
        reference:              referenceSAXFunc,
        characters:             charactersSAXFunc,
        ignorableWhitespace:    ignorableWhitespaceSAXFunc,
        processingInstruction:  processingInstructionSAXFunc,
        comment:                commentSAXFunc,
        warning:                warningSAXFunc,
        error:                  errorSAXFunc,
        fatalError:             fatalErrorSAXFunc,
        getParameterEntity:     getParameterEntitySAXFunc,
        cdataBlock:             cdataBlockSAXFunc,
        externalSubset:         externalSubsetSAXFunc,
        initialized:            c_uint,
        _private:               *c_void,
        startElementNs:         startElementNsSAX2Func,
        endElementNs:           endElementNsSAX2Func,
        serror:                 xmlStructuredErrorFunc,
    }

    #[link_args="-lxml2"]
    pub extern "C" {
        fn xmlCleanupParser();
        fn xmlSAXUserParseMemory(sax: *xmlSAXHandler, user_data: *c_void, buffer: *c_char, size: c_int) -> c_int;
    }
}

struct Registry {
    enums: ~[Enums],
    commands: ~[Commands]
}

struct Enums {
    namespace: ~str,
    etype: ~str,
    enums: ~[Enum],
}

struct Enum {
    name: ~str,
    value: ~str,
}

struct Commands {
    namespace: ~str,
    commands: ~[Command],
}

struct Command {
    name: ~str,
    rtype: ~str,
    params: ~[Param]
}

struct Param {
    ptype: ~str,
    name: ~str,
}

impl Registry {
    pub fn parse(xml: &str) -> Registry {
        do xml.as_c_str |data| {
            let sax_handler = &libxml2::xmlSAXHandler {
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
                warning:                warning,
                error:                  error,
                fatalError:             null(),
                getParameterEntity:     null(),
                cdataBlock:             null(),
                externalSubset:         null(),
                initialized:            libxml2::XML_SAX2_MAGIC,
                _private:               null(),
                startElementNs:         null(),
                endElementNs:           null(),
                serror:                 null(),
            };

            let registry = Registry {
                enums: ~[],
                commands: ~[],
            };
            unsafe {
                libxml2::xmlSAXUserParseMemory(sax_handler,
                                               transmute(&registry),
                                               data, xml.len() as c_int);
                libxml2::xmlCleanupParser();
            }
            registry
        }
    }

    pub unsafe fn from_c_void(ptr: *c_void) -> &mut Registry {
        transmute(ptr)
    }
}

extern "C" fn warning(_ctx: *c_void, msg: *c_char) {
    unsafe { println(fmt!("warning: %s", from_c_str(msg))) }
}

extern "C" fn error(_ctx: *c_void, msg: *c_char) {
    unsafe { println(fmt!("error: %s", from_c_str(msg))) }
}

extern "C" fn start_element(_ctx: *c_void, name: *libxml2::xmlChar, _atts: **libxml2::xmlChar) {
    unsafe { println(fmt!("start_element: %s", from_buf(name))) }
}

extern "C" fn end_element(_ctx: *c_void, name: *libxml2::xmlChar) {
    unsafe { println(fmt!("end_element: %s", from_buf(name))) }
}

extern "C" fn characters(_ctx: *c_void, ch: *libxml2::xmlChar, len: c_int) {
    unsafe { println(fmt!("characters: %s", from_buf_len(ch, len as uint))) }
}

fn main() {
    let _ = Registry::parse("<enum name=\"foo\">hiya</enum>");
}
