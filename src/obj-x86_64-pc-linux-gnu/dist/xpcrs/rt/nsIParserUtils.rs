//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/parser/html/nsIParserUtils.idl
//


/// `interface nsIParserUtils : nsISupports`
///

/// ```text
/// /**
///  * Non-Web HTML parser functionality to Firefox extensions and XULRunner apps.
///  * Don't use this from within Gecko--use nsContentUtils, nsTreeSanitizer, etc.
///  * directly instead.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIParserUtils {
    vtable: *const nsIParserUtilsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIParserUtils.
unsafe impl XpCom for nsIParserUtils {
    const IID: nsIID = nsID(0xa1101145, 0x0025, 0x411e,
        [0x88, 0x73, 0xfd, 0xf5, 0x7b, 0xf2, 0x81, 0x28]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIParserUtils {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIParserUtils.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIParserUtilsCoerce {
    /// Cheaply cast a value of this type from a `nsIParserUtils`.
    fn coerce_from(v: &nsIParserUtils) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIParserUtilsCoerce for nsIParserUtils {
    #[inline]
    fn coerce_from(v: &nsIParserUtils) -> &Self {
        v
    }
}

impl nsIParserUtils {
    /// Cast this `nsIParserUtils` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIParserUtilsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIParserUtils {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsISupportsCoerce> nsIParserUtilsCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIParserUtils) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIParserUtils
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIParserUtilsVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* AString sanitize (in AString src, in unsigned long flags); */
    pub Sanitize: unsafe extern "system" fn (this: *const nsIParserUtils, src: *const ::nsstring::nsAString, flags: u32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* AString convertToPlainText (in AString src, in unsigned long flags, in unsigned long wrapCol); */
    pub ConvertToPlainText: unsafe extern "system" fn (this: *const nsIParserUtils, src: *const ::nsstring::nsAString, flags: u32, wrapCol: u32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* DocumentFragment parseFragment (in AString fragment, in unsigned long flags, in boolean isXML, in nsIURI baseURI, in Element element); */
    pub ParseFragment: unsafe extern "system" fn (this: *const nsIParserUtils, fragment: *const ::nsstring::nsAString, flags: u32, isXML: bool, baseURI: *const nsIURI, element: *const libc::c_void, _retval: *mut *const libc::c_void) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIParserUtils {
    /// ```text
    /// /**
    ///    * Flag for sanitizer: Allow comment nodes.
    ///    */
    /// ```
    ///

    pub const SanitizerAllowComments: i64 = 1;

    /// ```text
    /// /**
    ///    * Flag for sanitizer: Allow <style> and style="" (with contents sanitized
        ///    * in case of -moz-binding). Note! If -moz-binding is absent, properties
    ///    * that might be XSS risks in other Web engines are preserved!
    ///    */
    /// ```
    ///

    pub const SanitizerAllowStyle: i64 = 2;

    /// ```text
    /// /**
    ///    * Flag for sanitizer: Only allow cid: URLs for embedded content.
    ///    *
    ///    * At present, sanitizing CSS backgrounds, etc., is not supported, so setting
    ///    * this together with SanitizerAllowStyle doesn't make sense.
    ///    *
    ///    * At present, sanitizing CSS syntax in SVG presentational attributes is not
    ///    * supported, so this option flattens out SVG.
    ///    */
    /// ```
    ///

    pub const SanitizerCidEmbedsOnly: i64 = 4;

    /// ```text
    /// /**
    ///    * Flag for sanitizer: Drop non-CSS presentational HTML elements and
    ///    * attributes, such as <font>, <center> and bgcolor="".
    ///    */
    /// ```
    ///

    pub const SanitizerDropNonCSSPresentation: i64 = 8;

    /// ```text
    /// /**
    ///    * Flag for sanitizer: Drop forms and form controls (excluding
        ///    * fieldset/legend).
    ///    */
    /// ```
    ///

    pub const SanitizerDropForms: i64 = 16;

    /// ```text
    /// /**
    ///    * Flag for sanitizer: Drop <img>, <video>, <audio> and <source> and flatten
    ///    * out SVG.
    ///    */
    /// ```
    ///

    pub const SanitizerDropMedia: i64 = 32;

    /// ```text
    /// /**
    ///    * Flag for sanitizer: Log messages to the console for everything that gets
    ///    * sanitized
    ///    */
    /// ```
    ///

    pub const SanitizerLogRemovals: i64 = 64;

    /// ```text
    /// /**
    ///    * Flag for sanitizer: Only CSS conditional rules will be moved,
    ///    * nothing else will be changed.
    ///    * Can be combined with flag SanitizerLogRemovals, only.
    ///    */
    /// ```
    ///

    pub const SanitizerRemoveOnlyConditionalCSS: i64 = 128;

    /// ```text
    /// /**
    ///    * Parses a string into an HTML document, sanitizes the document and
    ///    * returns the result serialized to a string.
    ///    *
    ///    * The sanitizer is designed to protect against XSS when sanitized content
    ///    * is inserted into a different-origin context without an iframe-equivalent
    ///    * sandboxing mechanism.
    ///    *
    ///    * By default, the sanitizer doesn't try to avoid leaking information that
    ///    * the content was viewed to third parties. That is, by default, e.g.
    ///    * <img src> pointing to an HTTP server potentially controlled by a third
    ///    * party is not removed. To avoid ambient information leakage upon loading
    ///    * the sanitized content, use the SanitizerInternalEmbedsOnly flag. In that
    ///    * case, <a href> links (and similar) to other content are preserved, so an
    ///    * explicit user action (following a link) after the content has been loaded
    ///    * can still leak information.
    ///    *
    ///    * By default, non-dangerous non-CSS presentational HTML elements and
    ///    * attributes or forms are not removed. To remove these, use
    ///    * SanitizerDropNonCSSPresentation and/or SanitizerDropForms.
    ///    *
    ///    * By default, comments and CSS is removed. To preserve comments, use
    ///    * SanitizerAllowComments. To preserve <style> and style="", use
    ///    * SanitizerAllowStyle. -moz-binding is removed from <style> and style="" if
    ///    * present. In this case, properties that Gecko doesn't recognize can get
    ///    * removed as a side effect. Note! If -moz-binding is not present, <style>
    ///    * and style="" and SanitizerAllowStyle is specified, the sanitized content
    ///    * may still be XSS dangerous if loaded into a non-Gecko Web engine!
    ///    *
    ///    * @param src the HTML source to parse (C++ callers are allowed but not
        ///    *            required to use the same string for the return value.)
    ///    * @param flags sanitization option flags defined above
    ///    */
    /// ```
    ///

    /// `AString sanitize (in AString src, in unsigned long flags);`
    #[inline]
    pub unsafe fn Sanitize(&self, src: *const ::nsstring::nsAString, flags: u32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).Sanitize)(self, src, flags, _retval)
    }


    /// ```text
    /// /**
    ///    * Convert HTML to plain text.
    ///    *
    ///    * @param src the HTML source to parse (C++ callers are allowed but not
        ///    *            required to use the same string for the return value.)
    ///    * @param flags conversion option flags defined in nsIDocumentEncoder
    ///    * @param wrapCol number of characters per line; 0 for no auto-wrapping
    ///    */
    /// ```
    ///

    /// `AString convertToPlainText (in AString src, in unsigned long flags, in unsigned long wrapCol);`
    #[inline]
    pub unsafe fn ConvertToPlainText(&self, src: *const ::nsstring::nsAString, flags: u32, wrapCol: u32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).ConvertToPlainText)(self, src, flags, wrapCol, _retval)
    }


    /// ```text
    /// /**
    ///    * Parses markup into a sanitized document fragment.
    ///    *
    ///    * @param fragment the input markup
    ///    * @param flags sanitization option flags defined above
    ///    * @param isXML true if |fragment| is XML and false if HTML
    ///    * @param baseURI the base URL for this fragment
    ///    * @param element the context node for the fragment parsing algorithm
    ///    */
    /// ```
    ///

    /// `DocumentFragment parseFragment (in AString fragment, in unsigned long flags, in boolean isXML, in nsIURI baseURI, in Element element);`
    #[inline]
    pub unsafe fn ParseFragment(&self, fragment: *const ::nsstring::nsAString, flags: u32, isXML: bool, baseURI: *const nsIURI, element: *const libc::c_void, _retval: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).ParseFragment)(self, fragment, flags, isXML, baseURI, element, _retval)
    }


}


