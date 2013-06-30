use std::cast::transmute;
use std::libc::*;
use std::str::raw::from_buf;
use extra::term;

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

pub struct xmlError {
    /// What part of the library raised this error
    domain: c_int,
    /// The error code, e.g. an xmlParserError
    code: c_int,
    /// human-readable informative error messag
    message: *c_char,
    /// how consequent is the error
    level: xmlErrorLevel,
    /// the filename
    file: *c_char,
    /// the line number if available
    line: c_int,
    /// extra string information
    str1: *c_char,
    /// extra string information
    str2: *c_char,
    /// extra string information
    str3: *c_char,
    /// extra number information
    int1: c_int,
    /// column number of the error or 0 if N/A
    int2: c_int,
    /// the parser context if available
    ctxt: *c_void,
    /// the node in the tree
    node: *c_void,
}

/// Error level type alias
priv type xmlErrorLevel = c_int;

/// No error
priv static XML_ERR_NONE:    xmlErrorLevel = 0;
/// A simple warning
priv static XML_ERR_WARNING: xmlErrorLevel = 1;
/// A recoverable error
priv static XML_ERR_ERROR:   xmlErrorLevel = 2;
/// A fatal error
priv static XML_ERR_FATAL:   xmlErrorLevel = 3;

#[link_args="-lxml2"]
extern "C" {
    priv fn xmlCleanupParser();
    priv fn xmlSAXUserParseMemory(sax: *xmlSAXHandler,
                                  user_data: *c_void,
                                  buffer: *c_char,
                                  size: c_int) -> c_int;
}

pub unsafe fn parse_str<T>(sax: &xmlSAXHandler, user_data: &T, src: &str) {
    do src.as_c_str |c_str| {
        xmlSAXUserParseMemory(sax, transmute(user_data), c_str, src.len() as c_int);
        xmlCleanupParser();
    }
}

/// The severity of the error
pub enum ErrorLevel {
    /// A simple warning
    Warning,
    /// A recoverable error
    Error,
    /// A fatal error
    Fatal,
}

impl ErrorLevel {
    fn from_libxml2_constant(value: xmlErrorLevel) -> Option<ErrorLevel> {
        match value {
            XML_ERR_WARNING => Some(Warning),
            XML_ERR_ERROR   => Some(Error),
            XML_ERR_FATAL   => Some(Fatal),
            _               => None,
        }
    }

    pub fn term_color(&self) -> u16 {
        match *self {
            Warning => term::color::yellow,
            Error   => term::color::red,
            Fatal   => term::color::red,
        }
    }

    pub fn to_color_str(&self) -> ~str {
        use std::io::with_str_writer;
        do with_str_writer |writer| {
            let term = term::Terminal::new(writer);
            term.map(|t| t.fg(self.term_color()));
            writer.write_str(self.to_str());
            term.map(|t| t.fg(term::color::black));
        }
    }
}

impl ToStr for ErrorLevel {
    pub fn to_str(&self) -> ~str {
        match *self {
            Warning => ~"Warning",
            Error   => ~"Error",
            Fatal   => ~"Fatal",
        }
    }
}

/// An xml parse error
pub struct ErrorData {
    level: ErrorLevel,
    line: uint,
    column: uint,
    message: ~str,
}

impl ErrorData {
    pub unsafe fn from_ptr(error: *xmlError) -> Option<~ErrorData> {
        use std::str::raw::from_c_str;
        do ErrorLevel::from_libxml2_constant((*error).level).map |&level| {
            ~ErrorData {
                level:      level,
                message:    from_c_str((*error).message),
                line:       (*error).line as uint,
                column:     (*error).int2 as uint,
            }
        }
    }
}

impl ToStr for ErrorData {
    pub fn to_str(&self) -> ~str {
        fmt!("%?:%? %s: %s",
             self.line,
             self.column,
             self.level.to_color_str(),
             self.message)
    }
}

pub fn build_attrib_vec(atts: **xmlChar) -> ~[(~str, ~str)] {
    use std::vec;
    do vec::build |push| {
        let mut curr = atts;
        unsafe {
            while !curr.is_null() && !(*curr).is_null() {
                push((
                    from_buf(*curr),
                    from_buf(*curr.offset(1))
                ));
                curr = curr.offset(2);
            }
        }
    }
}
