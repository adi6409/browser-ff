//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/media/webvtt/nsIWebVTTParserWrapper.idl
//


/// `interface nsIWebVTTParserWrapper : nsISupports`
///

/// ```text
/// /**
///  * Interface for a wrapper of a JS WebVTT parser (vtt.js).
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIWebVTTParserWrapper {
    vtable: *const nsIWebVTTParserWrapperVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIWebVTTParserWrapper.
unsafe impl XpCom for nsIWebVTTParserWrapper {
    const IID: nsIID = nsID(0x8dfe016e, 0x1701, 0x4618,
        [0x9f, 0x5e, 0x9a, 0x61, 0x54, 0xe8, 0x53, 0xf0]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIWebVTTParserWrapper {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIWebVTTParserWrapper.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIWebVTTParserWrapperCoerce {
    /// Cheaply cast a value of this type from a `nsIWebVTTParserWrapper`.
    fn coerce_from(v: &nsIWebVTTParserWrapper) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIWebVTTParserWrapperCoerce for nsIWebVTTParserWrapper {
    #[inline]
    fn coerce_from(v: &nsIWebVTTParserWrapper) -> &Self {
        v
    }
}

impl nsIWebVTTParserWrapper {
    /// Cast this `nsIWebVTTParserWrapper` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIWebVTTParserWrapperCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIWebVTTParserWrapper {
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
impl<T: nsISupportsCoerce> nsIWebVTTParserWrapperCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWebVTTParserWrapper) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIWebVTTParserWrapper
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIWebVTTParserWrapperVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void loadParser (in mozIDOMWindow window); */
    pub LoadParser: unsafe extern "system" fn (this: *const nsIWebVTTParserWrapper, window: *const mozIDOMWindow) -> ::nserror::nsresult,

    /* void parse (in ACString data); */
    pub Parse: unsafe extern "system" fn (this: *const nsIWebVTTParserWrapper, data: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* void flush (); */
    pub Flush: unsafe extern "system" fn (this: *const nsIWebVTTParserWrapper) -> ::nserror::nsresult,

    /* void watch (in nsIWebVTTListener callback); */
    pub Watch: unsafe extern "system" fn (this: *const nsIWebVTTParserWrapper, callback: *const nsIWebVTTListener) -> ::nserror::nsresult,

    /* void cancel (); */
    pub Cancel: unsafe extern "system" fn (this: *const nsIWebVTTParserWrapper) -> ::nserror::nsresult,

    /* DocumentFragment convertCueToDOMTree (in mozIDOMWindow window, in nsISupports cue); */
    pub ConvertCueToDOMTree: unsafe extern "system" fn (this: *const nsIWebVTTParserWrapper, window: *const mozIDOMWindow, cue: *const nsISupports, _retval: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* void processCues (in mozIDOMWindow window, in nsIVariant cues, in nsISupports overlay, in nsISupports controls); */
    pub ProcessCues: unsafe extern "system" fn (this: *const nsIWebVTTParserWrapper, window: *const mozIDOMWindow, cues: *const nsIVariant, overlay: *const nsISupports, controls: *const nsISupports) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIWebVTTParserWrapper {

    /// ```text
    /// /**
    ///    * Loads the JS WebVTTParser and sets it to use the passed window to create
    ///    * VTTRegions and VTTCues. This function must be called before calling
    ///    * parse, flush, or watch.
    ///    *
    ///    * @param window The window that the parser will use to create VTTCues and
    ///    *               VTTRegions.
    ///    *
    ///    */
    /// ```
    ///

    /// `void loadParser (in mozIDOMWindow window);`
    #[inline]
    pub unsafe fn LoadParser(&self, window: *const mozIDOMWindow) -> ::nserror::nsresult {
        ((*self.vtable).LoadParser)(self, window)
    }


    /// ```text
    /// /**
    ///    * Attempts to parse the stream's data as WebVTT format. When it successfully
    ///    * parses a WebVTT region or WebVTT cue it will create a VTTRegion or VTTCue
    ///    * object and pass it back to the callee through its callbacks.
    ///    *
    ///    * @param data   The buffer that contains the WebVTT data received by the
    ///    *               Necko consumer so far.
    ///    */
    /// ```
    ///

    /// `void parse (in ACString data);`
    #[inline]
    pub unsafe fn Parse(&self, data: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).Parse)(self, data)
    }


    /// ```text
    /// /**
    ///    * Flush indicates that no more data is expected from the stream. As such the
    ///    * parser should try to parse any kind of partial data it has.
    ///    */
    /// ```
    ///

    /// `void flush ();`
    #[inline]
    pub unsafe fn Flush(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Flush)(self, )
    }


    /// ```text
    /// /**
    ///    * Set this parser object to use an nsIWebVTTListener object for its onCue
    ///    * and onRegion callbacks.
    ///    *
    ///    * @param callback The nsIWebVTTListener object that exposes onCue and
    ///    *                 onRegion callbacks for the parser.
    ///    */
    /// ```
    ///

    /// `void watch (in nsIWebVTTListener callback);`
    #[inline]
    pub unsafe fn Watch(&self, callback: *const nsIWebVTTListener) -> ::nserror::nsresult {
        ((*self.vtable).Watch)(self, callback)
    }


    /// ```text
    /// /**
    ///    * Cancel watching notifications which parser would send.
    ///    */
    /// ```
    ///

    /// `void cancel ();`
    #[inline]
    pub unsafe fn Cancel(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Cancel)(self, )
    }


    /// ```text
    /// /**
    ///    * Convert the text content of a WebVTT cue to a document fragment so that
    ///    * we can display it on the page.
    ///    *
    ///    * @param window A window object with which the document fragment will be
    ///    *               created.
    ///    * @param cue    The cue whose content will be converted to a document
    ///    *               fragment.
    ///    */
    /// ```
    ///

    /// `DocumentFragment convertCueToDOMTree (in mozIDOMWindow window, in nsISupports cue);`
    #[inline]
    pub unsafe fn ConvertCueToDOMTree(&self, window: *const mozIDOMWindow, cue: *const nsISupports, _retval: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).ConvertCueToDOMTree)(self, window, cue, _retval)
    }


    /// ```text
    /// /**
    ///    * Compute the display state of the VTTCues in cues along with any VTTRegions
    ///    * that they might be in. First, it computes the positioning and styling of
    ///    * the cues and regions passed and converts them into a DOM tree rooted at
    ///    * a containing HTMLDivElement. It then adjusts those computed divs for
    ///    * overlap avoidance using the dimensions of 'overlay'. Finally, it adds the
    ///    * computed divs to the VTTCues display state property for use later.
    ///    *
    ///    * @param window  A window object with which it will create the DOM tree
    ///    *                and containing div element.
    ///    * @param cues    An array of VTTCues who need there display state to be
    ///    *                computed.
    ///    * @param overlay The HTMLElement that the cues will be displayed within.
    ///    * @param controls The video control element that will affect cues position.
    ///    */
    /// ```
    ///

    /// `void processCues (in mozIDOMWindow window, in nsIVariant cues, in nsISupports overlay, in nsISupports controls);`
    #[inline]
    pub unsafe fn ProcessCues(&self, window: *const mozIDOMWindow, cues: *const nsIVariant, overlay: *const nsISupports, controls: *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).ProcessCues)(self, window, cues, overlay, controls)
    }


}


