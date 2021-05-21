//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/base/nsICycleCollectorListener.idl
//


/// `interface nsICycleCollectorHandler : nsISupports`
///

/// ```text
/// /**
///  * A set of interfaces for recording the cycle collector's work. An instance
///  * of nsICycleCollectorListener can be configured to enable various
///  * options, then passed to the cycle collector when it runs.
///  * Note that additional logging options are available by setting environment
///  * variables, as described at the top of nsCycleCollector.cpp.
///  */
/// /**
///  * nsICycleCollectorHandler is the interface JS code should implement to
///  * receive the results logged by an nsICycleCollectorListener
///  * instance. Pass an instance of this to the logger's 'processNext' method
///  * after the collection has run. This will describe the objects the cycle
///  * collector visited, the edges it found, and the conclusions it reached
///  * about the liveness of objects.
///  *
///  * In more detail:
///  * - For each node in the graph:
///  *   - a call is made to either |noteRefCountedObject| or |noteGCedObject|, to
///  *     describe the node itself; and
///  *   - for each edge starting at that node, a call is made to |noteEdge|.
///  *
///  * - Then, a series of calls are made to:
///  *   - |describeRoot|, for reference-counted nodes that the CC has identified as
///  *     being alive because there are unknown references to those nodes.
///  *   - |describeGarbage|, for nodes the cycle collector has identified as garbage.
///  *
///  *   Any node not mentioned in a call to |describeRoot| or |describeGarbage| is
///  *   neither a root nor garbage. The cycle collector was able to find all of the
///  *   edges implied by the node's reference count.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsICycleCollectorHandler {
    vtable: *const nsICycleCollectorHandlerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsICycleCollectorHandler.
unsafe impl XpCom for nsICycleCollectorHandler {
    const IID: nsIID = nsID(0x7f093367, 0x1492, 0x4b89,
        [0x87, 0xaf, 0xc0, 0x1d, 0xbc, 0x83, 0x12, 0x46]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsICycleCollectorHandler {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsICycleCollectorHandler.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsICycleCollectorHandlerCoerce {
    /// Cheaply cast a value of this type from a `nsICycleCollectorHandler`.
    fn coerce_from(v: &nsICycleCollectorHandler) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsICycleCollectorHandlerCoerce for nsICycleCollectorHandler {
    #[inline]
    fn coerce_from(v: &nsICycleCollectorHandler) -> &Self {
        v
    }
}

impl nsICycleCollectorHandler {
    /// Cast this `nsICycleCollectorHandler` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsICycleCollectorHandlerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsICycleCollectorHandler {
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
impl<T: nsISupportsCoerce> nsICycleCollectorHandlerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICycleCollectorHandler) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsICycleCollectorHandler
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsICycleCollectorHandlerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void noteRefCountedObject (in ACString aAddress, in unsigned long aRefCount, in ACString aObjectDescription); */
    pub NoteRefCountedObject: unsafe extern "system" fn (this: *const nsICycleCollectorHandler, aAddress: *const ::nsstring::nsACString, aRefCount: u32, aObjectDescription: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* void noteGCedObject (in ACString aAddress, in boolean aMarked, in ACString aObjectDescription, in ACString aCompartmentAddress); */
    pub NoteGCedObject: unsafe extern "system" fn (this: *const nsICycleCollectorHandler, aAddress: *const ::nsstring::nsACString, aMarked: bool, aObjectDescription: *const ::nsstring::nsACString, aCompartmentAddress: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* void noteEdge (in ACString aFromAddress, in ACString aToAddress, in ACString aEdgeName); */
    pub NoteEdge: unsafe extern "system" fn (this: *const nsICycleCollectorHandler, aFromAddress: *const ::nsstring::nsACString, aToAddress: *const ::nsstring::nsACString, aEdgeName: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* void describeRoot (in ACString aAddress, in unsigned long aKnownEdges); */
    pub DescribeRoot: unsafe extern "system" fn (this: *const nsICycleCollectorHandler, aAddress: *const ::nsstring::nsACString, aKnownEdges: u32) -> ::nserror::nsresult,

    /* void describeGarbage (in ACString aAddress); */
    pub DescribeGarbage: unsafe extern "system" fn (this: *const nsICycleCollectorHandler, aAddress: *const ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsICycleCollectorHandler {


    /// `void noteRefCountedObject (in ACString aAddress, in unsigned long aRefCount, in ACString aObjectDescription);`
    #[inline]
    pub unsafe fn NoteRefCountedObject(&self, aAddress: *const ::nsstring::nsACString, aRefCount: u32, aObjectDescription: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).NoteRefCountedObject)(self, aAddress, aRefCount, aObjectDescription)
    }



    /// `void noteGCedObject (in ACString aAddress, in boolean aMarked, in ACString aObjectDescription, in ACString aCompartmentAddress);`
    #[inline]
    pub unsafe fn NoteGCedObject(&self, aAddress: *const ::nsstring::nsACString, aMarked: bool, aObjectDescription: *const ::nsstring::nsACString, aCompartmentAddress: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).NoteGCedObject)(self, aAddress, aMarked, aObjectDescription, aCompartmentAddress)
    }



    /// `void noteEdge (in ACString aFromAddress, in ACString aToAddress, in ACString aEdgeName);`
    #[inline]
    pub unsafe fn NoteEdge(&self, aFromAddress: *const ::nsstring::nsACString, aToAddress: *const ::nsstring::nsACString, aEdgeName: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).NoteEdge)(self, aFromAddress, aToAddress, aEdgeName)
    }



    /// `void describeRoot (in ACString aAddress, in unsigned long aKnownEdges);`
    #[inline]
    pub unsafe fn DescribeRoot(&self, aAddress: *const ::nsstring::nsACString, aKnownEdges: u32) -> ::nserror::nsresult {
        ((*self.vtable).DescribeRoot)(self, aAddress, aKnownEdges)
    }



    /// `void describeGarbage (in ACString aAddress);`
    #[inline]
    pub unsafe fn DescribeGarbage(&self, aAddress: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).DescribeGarbage)(self, aAddress)
    }


}


/// `interface nsICycleCollectorLogSink : nsISupports`
///

/// ```text
/// /**
///  * This interface allows replacing the log-writing backend for an
///  * nsICycleCollectorListener.  As this interface is also called while
///  * the cycle collector is running, it cannot be implemented in JS.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsICycleCollectorLogSink {
    vtable: *const nsICycleCollectorLogSinkVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsICycleCollectorLogSink.
unsafe impl XpCom for nsICycleCollectorLogSink {
    const IID: nsIID = nsID(0x3ad9875f, 0xd0e4, 0x4ac2,
        [0x87, 0xe3, 0xf1, 0x27, 0xf6, 0xc0, 0x2c, 0xe1]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsICycleCollectorLogSink {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsICycleCollectorLogSink.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsICycleCollectorLogSinkCoerce {
    /// Cheaply cast a value of this type from a `nsICycleCollectorLogSink`.
    fn coerce_from(v: &nsICycleCollectorLogSink) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsICycleCollectorLogSinkCoerce for nsICycleCollectorLogSink {
    #[inline]
    fn coerce_from(v: &nsICycleCollectorLogSink) -> &Self {
        v
    }
}

impl nsICycleCollectorLogSink {
    /// Cast this `nsICycleCollectorLogSink` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsICycleCollectorLogSinkCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsICycleCollectorLogSink {
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
impl<T: nsISupportsCoerce> nsICycleCollectorLogSinkCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICycleCollectorLogSink) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsICycleCollectorLogSink
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsICycleCollectorLogSinkVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [noscript] void open (out FILE aGCLog, out FILE aCCLog); */
    /// Unable to generate binding because `native type FILE unsupported`
    pub Open: *const ::libc::c_void,

    /* void closeGCLog (); */
    pub CloseGCLog: unsafe extern "system" fn (this: *const nsICycleCollectorLogSink) -> ::nserror::nsresult,

    /* void closeCCLog (); */
    pub CloseCCLog: unsafe extern "system" fn (this: *const nsICycleCollectorLogSink) -> ::nserror::nsresult,

    /* attribute AString filenameIdentifier; */
    pub GetFilenameIdentifier: unsafe extern "system" fn (this: *const nsICycleCollectorLogSink, aFilenameIdentifier: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString filenameIdentifier; */
    pub SetFilenameIdentifier: unsafe extern "system" fn (this: *const nsICycleCollectorLogSink, aFilenameIdentifier: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute int32_t processIdentifier; */
    pub GetProcessIdentifier: unsafe extern "system" fn (this: *const nsICycleCollectorLogSink, aProcessIdentifier: *mut int32_t) -> ::nserror::nsresult,

    /* attribute int32_t processIdentifier; */
    pub SetProcessIdentifier: unsafe extern "system" fn (this: *const nsICycleCollectorLogSink, aProcessIdentifier: int32_t) -> ::nserror::nsresult,

    /* readonly attribute nsIFile gcLog; */
    pub GetGcLog: unsafe extern "system" fn (this: *const nsICycleCollectorLogSink, aGcLog: *mut*const nsIFile) -> ::nserror::nsresult,

    /* readonly attribute nsIFile ccLog; */
    pub GetCcLog: unsafe extern "system" fn (this: *const nsICycleCollectorLogSink, aCcLog: *mut*const nsIFile) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsICycleCollectorLogSink {


    /// `[noscript] void open (out FILE aGCLog, out FILE aCCLog);`
    const _Open: () = ();


    /// `void closeGCLog ();`
    #[inline]
    pub unsafe fn CloseGCLog(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).CloseGCLog)(self, )
    }



    /// `void closeCCLog ();`
    #[inline]
    pub unsafe fn CloseCCLog(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).CloseCCLog)(self, )
    }



    /// `attribute AString filenameIdentifier;`
    #[inline]
    pub unsafe fn GetFilenameIdentifier(&self, aFilenameIdentifier: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetFilenameIdentifier)(self, aFilenameIdentifier)
    }



    /// `attribute AString filenameIdentifier;`
    #[inline]
    pub unsafe fn SetFilenameIdentifier(&self, aFilenameIdentifier: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetFilenameIdentifier)(self, aFilenameIdentifier)
    }



    /// `attribute int32_t processIdentifier;`
    #[inline]
    pub unsafe fn GetProcessIdentifier(&self, aProcessIdentifier: *mut int32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetProcessIdentifier)(self, aProcessIdentifier)
    }



    /// `attribute int32_t processIdentifier;`
    #[inline]
    pub unsafe fn SetProcessIdentifier(&self, aProcessIdentifier: int32_t) -> ::nserror::nsresult {
        ((*self.vtable).SetProcessIdentifier)(self, aProcessIdentifier)
    }



    /// `readonly attribute nsIFile gcLog;`
    #[inline]
    pub unsafe fn GetGcLog(&self, aGcLog: *mut*const nsIFile) -> ::nserror::nsresult {
        ((*self.vtable).GetGcLog)(self, aGcLog)
    }



    /// `readonly attribute nsIFile ccLog;`
    #[inline]
    pub unsafe fn GetCcLog(&self, aCcLog: *mut*const nsIFile) -> ::nserror::nsresult {
        ((*self.vtable).GetCcLog)(self, aCcLog)
    }


}


/// `interface nsICycleCollectorListener : nsISupports`
///

/// ```text
/// /**
///  * This interface is used to configure some reporting options for the cycle
///  * collector. This interface cannot be implemented by JavaScript code, as it
///  * is called while the cycle collector is running.
///  *
///  * To analyze cycle collection data in JS:
///  *
///  * - Create an instance of nsICycleCollectorListener, which implements this
///  *   interface. In C++, this can be done by calling
///  *   nsCycleCollector_createLogger(). In JS, this can be done by calling
///  *   Components.utils.createCCLogger().
///  *
///  * - Set its |disableLog| property to true. This prevents the logger from
///  *   printing messages about each method call to a temporary log file.
///  *
///  * - Set its |wantAfterProcessing| property to true. This tells the logger
///  *   to record calls to its methods in memory. The |processNext| method
///  *   returns events from this record.
///  *
///  * - Perform a collection using the logger. For example, call
///  *   |nsIDOMWindowUtils|'s |garbageCollect| method, passing the logger as
///  *   the |aListener| argument.
///  *
///  * - When the collection is complete, loop calling the logger's
///  *   |processNext| method, passing a JavaScript object that implements
///  *   nsICycleCollectorHandler. This JS code is free to allocate and operate
///  *   on objects however it pleases: the cycle collector has finished its
///  *   work, and the JS code is simply consuming recorded data.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsICycleCollectorListener {
    vtable: *const nsICycleCollectorListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsICycleCollectorListener.
unsafe impl XpCom for nsICycleCollectorListener {
    const IID: nsIID = nsID(0x703b53b6, 0x24f6, 0x40c6,
        [0x9e, 0xa9, 0xae, 0xb2, 0xdc, 0x53, 0xd1, 0x70]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsICycleCollectorListener {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsICycleCollectorListener.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsICycleCollectorListenerCoerce {
    /// Cheaply cast a value of this type from a `nsICycleCollectorListener`.
    fn coerce_from(v: &nsICycleCollectorListener) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsICycleCollectorListenerCoerce for nsICycleCollectorListener {
    #[inline]
    fn coerce_from(v: &nsICycleCollectorListener) -> &Self {
        v
    }
}

impl nsICycleCollectorListener {
    /// Cast this `nsICycleCollectorListener` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsICycleCollectorListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsICycleCollectorListener {
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
impl<T: nsISupportsCoerce> nsICycleCollectorListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICycleCollectorListener) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsICycleCollectorListener
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsICycleCollectorListenerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* nsICycleCollectorListener allTraces (); */
    pub AllTraces: unsafe extern "system" fn (this: *const nsICycleCollectorListener, _retval: *mut *const nsICycleCollectorListener) -> ::nserror::nsresult,

    /* readonly attribute boolean wantAllTraces; */
    pub GetWantAllTraces: unsafe extern "system" fn (this: *const nsICycleCollectorListener, aWantAllTraces: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean disableLog; */
    pub GetDisableLog: unsafe extern "system" fn (this: *const nsICycleCollectorListener, aDisableLog: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean disableLog; */
    pub SetDisableLog: unsafe extern "system" fn (this: *const nsICycleCollectorListener, aDisableLog: bool) -> ::nserror::nsresult,

    /* attribute nsICycleCollectorLogSink logSink; */
    pub GetLogSink: unsafe extern "system" fn (this: *const nsICycleCollectorListener, aLogSink: *mut *const nsICycleCollectorLogSink) -> ::nserror::nsresult,

    /* attribute nsICycleCollectorLogSink logSink; */
    pub SetLogSink: unsafe extern "system" fn (this: *const nsICycleCollectorListener, aLogSink: *const nsICycleCollectorLogSink) -> ::nserror::nsresult,

    /* attribute boolean wantAfterProcessing; */
    pub GetWantAfterProcessing: unsafe extern "system" fn (this: *const nsICycleCollectorListener, aWantAfterProcessing: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean wantAfterProcessing; */
    pub SetWantAfterProcessing: unsafe extern "system" fn (this: *const nsICycleCollectorListener, aWantAfterProcessing: bool) -> ::nserror::nsresult,

    /* boolean processNext (in nsICycleCollectorHandler aHandler); */
    pub ProcessNext: unsafe extern "system" fn (this: *const nsICycleCollectorListener, aHandler: *const nsICycleCollectorHandler, _retval: *mut bool) -> ::nserror::nsresult,

    /* [noscript] nsCycleCollectorLoggerPtr asLogger (); */
    /// Unable to generate binding because `native type nsCycleCollectorLogger unsupported`
    pub AsLogger: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsICycleCollectorListener {


    /// `nsICycleCollectorListener allTraces ();`
    #[inline]
    pub unsafe fn AllTraces(&self, _retval: *mut *const nsICycleCollectorListener) -> ::nserror::nsresult {
        ((*self.vtable).AllTraces)(self, _retval)
    }



    /// `readonly attribute boolean wantAllTraces;`
    #[inline]
    pub unsafe fn GetWantAllTraces(&self, aWantAllTraces: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetWantAllTraces)(self, aWantAllTraces)
    }



    /// `attribute boolean disableLog;`
    #[inline]
    pub unsafe fn GetDisableLog(&self, aDisableLog: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetDisableLog)(self, aDisableLog)
    }



    /// `attribute boolean disableLog;`
    #[inline]
    pub unsafe fn SetDisableLog(&self, aDisableLog: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetDisableLog)(self, aDisableLog)
    }



    /// `attribute nsICycleCollectorLogSink logSink;`
    #[inline]
    pub unsafe fn GetLogSink(&self, aLogSink: *mut *const nsICycleCollectorLogSink) -> ::nserror::nsresult {
        ((*self.vtable).GetLogSink)(self, aLogSink)
    }



    /// `attribute nsICycleCollectorLogSink logSink;`
    #[inline]
    pub unsafe fn SetLogSink(&self, aLogSink: *const nsICycleCollectorLogSink) -> ::nserror::nsresult {
        ((*self.vtable).SetLogSink)(self, aLogSink)
    }



    /// `attribute boolean wantAfterProcessing;`
    #[inline]
    pub unsafe fn GetWantAfterProcessing(&self, aWantAfterProcessing: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetWantAfterProcessing)(self, aWantAfterProcessing)
    }



    /// `attribute boolean wantAfterProcessing;`
    #[inline]
    pub unsafe fn SetWantAfterProcessing(&self, aWantAfterProcessing: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetWantAfterProcessing)(self, aWantAfterProcessing)
    }



    /// `boolean processNext (in nsICycleCollectorHandler aHandler);`
    #[inline]
    pub unsafe fn ProcessNext(&self, aHandler: *const nsICycleCollectorHandler, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).ProcessNext)(self, aHandler, _retval)
    }



    /// `[noscript] nsCycleCollectorLoggerPtr asLogger ();`
    const _AsLogger: () = ();

}


