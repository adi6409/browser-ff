//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/test/httpserver/nsIHttpServer.idl
//


/// `interface nsIHttpServer : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIHttpServer {
    vtable: *const nsIHttpServerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIHttpServer.
unsafe impl XpCom for nsIHttpServer {
    const IID: nsIID = nsID(0xcea8812e, 0xfaa6, 0x4013,
        [0x93, 0x96, 0xf9, 0x93, 0x6c, 0xbb, 0x74, 0xec]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIHttpServer {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIHttpServer.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIHttpServerCoerce {
    /// Cheaply cast a value of this type from a `nsIHttpServer`.
    fn coerce_from(v: &nsIHttpServer) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIHttpServerCoerce for nsIHttpServer {
    #[inline]
    fn coerce_from(v: &nsIHttpServer) -> &Self {
        v
    }
}

impl nsIHttpServer {
    /// Cast this `nsIHttpServer` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIHttpServerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIHttpServer {
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
impl<T: nsISupportsCoerce> nsIHttpServerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIHttpServer) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIHttpServer
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIHttpServerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void start (in long port); */
    pub Start: unsafe extern "system" fn (this: *const nsIHttpServer, port: i32) -> ::nserror::nsresult,

    /* void stop (in nsIHttpServerStoppedCallback callback); */
    pub Stop: unsafe extern "system" fn (this: *const nsIHttpServer, callback: *const nsIHttpServerStoppedCallback) -> ::nserror::nsresult,

    /* void registerFile (in string path, in nsIFile file, [optional] in nsIHttpRequestHandler handler); */
    pub RegisterFile: unsafe extern "system" fn (this: *const nsIHttpServer, path: *const libc::c_char, file: *const nsIFile, handler: *const nsIHttpRequestHandler) -> ::nserror::nsresult,

    /* void registerPathHandler (in string path, in nsIHttpRequestHandler handler); */
    pub RegisterPathHandler: unsafe extern "system" fn (this: *const nsIHttpServer, path: *const libc::c_char, handler: *const nsIHttpRequestHandler) -> ::nserror::nsresult,

    /* void registerPrefixHandler (in string prefix, in nsIHttpRequestHandler handler); */
    pub RegisterPrefixHandler: unsafe extern "system" fn (this: *const nsIHttpServer, prefix: *const libc::c_char, handler: *const nsIHttpRequestHandler) -> ::nserror::nsresult,

    /* void registerErrorHandler (in unsigned long code, in nsIHttpRequestHandler handler); */
    pub RegisterErrorHandler: unsafe extern "system" fn (this: *const nsIHttpServer, code: u32, handler: *const nsIHttpRequestHandler) -> ::nserror::nsresult,

    /* void registerDirectory (in string path, in nsIFile dir); */
    pub RegisterDirectory: unsafe extern "system" fn (this: *const nsIHttpServer, path: *const libc::c_char, dir: *const nsIFile) -> ::nserror::nsresult,

    /* void registerContentType (in string extension, in string type); */
    pub RegisterContentType: unsafe extern "system" fn (this: *const nsIHttpServer, extension: *const libc::c_char, type_: *const libc::c_char) -> ::nserror::nsresult,

    /* void setIndexHandler (in nsIHttpRequestHandler handler); */
    pub SetIndexHandler: unsafe extern "system" fn (this: *const nsIHttpServer, handler: *const nsIHttpRequestHandler) -> ::nserror::nsresult,

    /* readonly attribute nsIHttpServerIdentity identity; */
    pub GetIdentity: unsafe extern "system" fn (this: *const nsIHttpServer, aIdentity: *mut*const nsIHttpServerIdentity) -> ::nserror::nsresult,

    /* AString getState (in AString path, in AString key); */
    pub GetState: unsafe extern "system" fn (this: *const nsIHttpServer, path: *const ::nsstring::nsAString, key: *const ::nsstring::nsAString, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void setState (in AString path, in AString key, in AString value); */
    pub SetState: unsafe extern "system" fn (this: *const nsIHttpServer, path: *const ::nsstring::nsAString, key: *const ::nsstring::nsAString, value: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* AString getSharedState (in AString key); */
    pub GetSharedState: unsafe extern "system" fn (this: *const nsIHttpServer, key: *const ::nsstring::nsAString, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void setSharedState (in AString key, in AString value); */
    pub SetSharedState: unsafe extern "system" fn (this: *const nsIHttpServer, key: *const ::nsstring::nsAString, value: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* nsISupports getObjectState (in AString key); */
    pub GetObjectState: unsafe extern "system" fn (this: *const nsIHttpServer, key: *const ::nsstring::nsAString, _retval: *mut *const nsISupports) -> ::nserror::nsresult,

    /* void setObjectState (in AString key, in nsISupports value); */
    pub SetObjectState: unsafe extern "system" fn (this: *const nsIHttpServer, key: *const ::nsstring::nsAString, value: *const nsISupports) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIHttpServer {

    /// ```text
    /// /**
    ///  * An interface which represents an HTTP server.
    ///  */
    /// /**
    ///    * Starts up this server, listening upon the given port.
    ///    *
    ///    * @param port
    ///    *   the port upon which listening should happen, or -1 if no specific port is
    ///    *   desired
    ///    * @throws NS_ERROR_ALREADY_INITIALIZED
    ///    *   if this server is already started
    ///    * @throws NS_ERROR_NOT_AVAILABLE
    ///    *   if the server is not started and cannot be started on the desired port
    ///    *   (perhaps because the port is already in use or because the process does
        ///    *   not have privileges to do so)
    ///    * @note
    ///    *   Behavior is undefined if this method is called after stop() has been
    ///    *   called on this but before the provided callback function has been
    ///    *   called.
    ///    */
    /// ```
    ///

    /// `void start (in long port);`
    #[inline]
    pub unsafe fn Start(&self, port: i32) -> ::nserror::nsresult {
        ((*self.vtable).Start)(self, port)
    }


    /// ```text
    /// /**
    ///    * Shuts down this server if it is running (including the period of time after
        ///    * stop() has been called but before the provided callback has been called).
    ///    *
    ///    * @param callback
    ///    *   an asynchronous callback used to notify the user when this server is
    ///    *   stopped and all pending requests have been fully served
    ///    * @throws NS_ERROR_NULL_POINTER
    ///    *   if callback is null
    ///    * @throws NS_ERROR_UNEXPECTED
    ///    *   if this server is not running
    ///    */
    /// ```
    ///

    /// `void stop (in nsIHttpServerStoppedCallback callback);`
    #[inline]
    pub unsafe fn Stop(&self, callback: *const nsIHttpServerStoppedCallback) -> ::nserror::nsresult {
        ((*self.vtable).Stop)(self, callback)
    }


    /// ```text
    /// /**
    ///    * Associates the local file represented by the string file with all requests
    ///    * which match request.
    ///    *
    ///    * @param path
    ///    *   the path which is to be mapped to the given file; must begin with "/" and
    ///    *   be a valid URI path (i.e., no query string, hash reference, etc.)
    ///    * @param file
    ///    *   the file to serve for the given path, or null to remove any mapping that
    ///    *   might exist; this file must exist for the lifetime of the server
    ///    * @param handler
    ///    *   an optional object which can be used to handle any further changes.
    ///    */
    /// ```
    ///

    /// `void registerFile (in string path, in nsIFile file, [optional] in nsIHttpRequestHandler handler);`
    #[inline]
    pub unsafe fn RegisterFile(&self, path: *const libc::c_char, file: *const nsIFile, handler: *const nsIHttpRequestHandler) -> ::nserror::nsresult {
        ((*self.vtable).RegisterFile)(self, path, file, handler)
    }


    /// ```text
    /// /**
    ///    * Registers a custom path handler.
    ///    *
    ///    * @param path
    ///    *   the path on the server (beginning with a "/") which is to be handled by
    ///    *   handler; this path must not include a query string or hash component; it
    ///    *   also should usually be canonicalized, since most browsers will do so
    ///    *   before sending otherwise-matching requests
    ///    * @param handler
    ///    *   an object which will handle any requests for the given path, or null to
    ///    *   remove any existing handler; if while the server is running the handler
    ///    *   throws an exception while responding to a request, an HTTP 500 response
    ///    *   will be returned
    ///    * @throws NS_ERROR_INVALID_ARG
    ///    *   if path does not begin with a "/"
    ///    */
    /// ```
    ///

    /// `void registerPathHandler (in string path, in nsIHttpRequestHandler handler);`
    #[inline]
    pub unsafe fn RegisterPathHandler(&self, path: *const libc::c_char, handler: *const nsIHttpRequestHandler) -> ::nserror::nsresult {
        ((*self.vtable).RegisterPathHandler)(self, path, handler)
    }


    /// ```text
    /// /**
    ///    * Registers a custom prefix handler.
    ///    *
    ///    * @param prefix
    ///    *   the path on the server (beginning and ending with "/") which is to be
    ///    *   handled by handler; this path must not include a query string or hash
    ///    *   component. All requests that start with this prefix will be directed to
    ///    *   the given handler.
    ///    * @param handler
    ///    *   an object which will handle any requests for the given path, or null to
    ///    *   remove any existing handler; if while the server is running the handler
    ///    *   throws an exception while responding to a request, an HTTP 500 response
    ///    *   will be returned
    ///    * @throws NS_ERROR_INVALID_ARG
    ///    *   if path does not begin with a "/" or does not end with a "/"
    ///    */
    /// ```
    ///

    /// `void registerPrefixHandler (in string prefix, in nsIHttpRequestHandler handler);`
    #[inline]
    pub unsafe fn RegisterPrefixHandler(&self, prefix: *const libc::c_char, handler: *const nsIHttpRequestHandler) -> ::nserror::nsresult {
        ((*self.vtable).RegisterPrefixHandler)(self, prefix, handler)
    }


    /// ```text
    /// /**
    ///    * Registers a custom error page handler.
    ///    *
    ///    * @param code
    ///    *   the error code which is to be handled by handler
    ///    * @param handler
    ///    *   an object which will handle any requests which generate the given status
    ///    *   code, or null to remove any existing handler.  If the handler throws an
    ///    *   exception during server operation, fallback is to the genericized error
    ///    *   handler (the x00 version), then to 500, using a user-defined error
    ///    *   handler if one exists or the server default handler otherwise.  Fallback
    ///    *   will never occur from a user-provided handler that throws to the same
    ///    *   handler as provided by the server, e.g. a throwing user 404 falls back to
    ///    *   400, not a server-provided 404 that might not throw.
    ///    * @note
    ///    *   If the error handler handles HTTP 500 and throws, behavior is undefined.
    ///    */
    /// ```
    ///

    /// `void registerErrorHandler (in unsigned long code, in nsIHttpRequestHandler handler);`
    #[inline]
    pub unsafe fn RegisterErrorHandler(&self, code: u32, handler: *const nsIHttpRequestHandler) -> ::nserror::nsresult {
        ((*self.vtable).RegisterErrorHandler)(self, code, handler)
    }


    /// ```text
    /// /**
    ///    * Maps all requests to paths beneath path to the corresponding file beneath
    ///    * dir.
    ///    *
    ///    * @param path
    ///    *   the absolute path on the server against which requests will be served
    ///    *   from dir (e.g., "/", "/foo/", etc.); must begin and end with a forward
    ///    *   slash
    ///    * @param dir
    ///    *   the directory to be used to serve all requests for paths underneath path
    ///    *   (except those further overridden by another, deeper path registered with
        ///    *   another directory); if null, any current mapping for the given path is
    ///    *   removed
    ///    * @throws NS_ERROR_INVALID_ARG
    ///    *   if dir is non-null and does not exist or is not a directory, or if path
    ///    *   does not begin with and end with a forward slash
    ///    */
    /// ```
    ///

    /// `void registerDirectory (in string path, in nsIFile dir);`
    #[inline]
    pub unsafe fn RegisterDirectory(&self, path: *const libc::c_char, dir: *const nsIFile) -> ::nserror::nsresult {
        ((*self.vtable).RegisterDirectory)(self, path, dir)
    }


    /// ```text
    /// /**
    ///    * Associates files with the given extension with the given Content-Type when
    ///    * served by this server, in the absence of any file-specific information
    ///    * about the desired Content-Type.  If type is empty, removes any extant
    ///    * mapping, if one is present.
    ///    *
    ///    * @throws NS_ERROR_INVALID_ARG
    ///    *   if the given type is not a valid header field value, i.e. if it doesn't
    ///    *   match the field-value production in RFC 2616
    ///    * @note
    ///    *   No syntax checking is done of the given type, beyond ensuring that it is
    ///    *   a valid header field value.  Behavior when not given a string matching
    ///    *   the media-type production in RFC 2616 section 3.7 is undefined.
    ///    *   Implementations may choose to define specific behavior for types which do
    ///    *   not match the production, such as for CGI functionality.
    ///    * @note
    ///    *   Implementations MAY treat type as a trusted argument; users who fail to
    ///    *   generate this string from trusted data risk security vulnerabilities.
    ///    */
    /// ```
    ///

    /// `void registerContentType (in string extension, in string type);`
    #[inline]
    pub unsafe fn RegisterContentType(&self, extension: *const libc::c_char, type_: *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).RegisterContentType)(self, extension, type_)
    }


    /// ```text
    /// /**
    ///    * Sets the handler used to display the contents of a directory if
    ///    * the directory contains no index page.
    ///    *
    ///    * @param handler
    ///    *   an object which will handle any requests for directories which
    ///    *   do not contain index pages, or null to reset to the default
    ///    *   index handler; if while the server is running the handler
    ///    *   throws an exception while responding to a request, an HTTP 500
    ///    *   response will be returned.  An nsIFile corresponding to the
    ///    *   directory is available from the metadata object passed to the
    ///    *   handler, under the key "directory".
    ///    */
    /// ```
    ///

    /// `void setIndexHandler (in nsIHttpRequestHandler handler);`
    #[inline]
    pub unsafe fn SetIndexHandler(&self, handler: *const nsIHttpRequestHandler) -> ::nserror::nsresult {
        ((*self.vtable).SetIndexHandler)(self, handler)
    }


    /// ```text
    /// /** Represents the locations at which this server is reachable. */
    /// ```
    ///

    /// `readonly attribute nsIHttpServerIdentity identity;`
    #[inline]
    pub unsafe fn GetIdentity(&self, aIdentity: *mut*const nsIHttpServerIdentity) -> ::nserror::nsresult {
        ((*self.vtable).GetIdentity)(self, aIdentity)
    }


    /// ```text
    /// /**
    ///    * Retrieves the string associated with the given key in this, for the given
    ///    * path's saved state.  All keys are initially associated with the empty
    ///    * string.
    ///    */
    /// ```
    ///

    /// `AString getState (in AString path, in AString key);`
    #[inline]
    pub unsafe fn GetState(&self, path: *const ::nsstring::nsAString, key: *const ::nsstring::nsAString, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetState)(self, path, key, _retval)
    }


    /// ```text
    /// /**
    ///    * Sets the string associated with the given key in this, for the given path's
    ///    * saved state.
    ///    */
    /// ```
    ///

    /// `void setState (in AString path, in AString key, in AString value);`
    #[inline]
    pub unsafe fn SetState(&self, path: *const ::nsstring::nsAString, key: *const ::nsstring::nsAString, value: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetState)(self, path, key, value)
    }


    /// ```text
    /// /**
    ///    * Retrieves the string associated with the given key in this, in
    ///    * entire-server saved state.  All keys are initially associated with the
    ///    * empty string.
    ///    */
    /// ```
    ///

    /// `AString getSharedState (in AString key);`
    #[inline]
    pub unsafe fn GetSharedState(&self, key: *const ::nsstring::nsAString, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetSharedState)(self, key, _retval)
    }


    /// ```text
    /// /**
    ///    * Sets the string associated with the given key in this, in entire-server
    ///    * saved state.
    ///    */
    /// ```
    ///

    /// `void setSharedState (in AString key, in AString value);`
    #[inline]
    pub unsafe fn SetSharedState(&self, key: *const ::nsstring::nsAString, value: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetSharedState)(self, key, value)
    }


    /// ```text
    /// /**
    ///    * Retrieves the object associated with the given key in this in
    ///    * object-valued saved state.  All keys are initially associated with null.
    ///    */
    /// ```
    ///

    /// `nsISupports getObjectState (in AString key);`
    #[inline]
    pub unsafe fn GetObjectState(&self, key: *const ::nsstring::nsAString, _retval: *mut *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).GetObjectState)(self, key, _retval)
    }


    /// ```text
    /// /**
    ///    * Sets the object associated with the given key in this in object-valued
    ///    * saved state.  The value may be null.
    ///    */
    /// ```
    ///

    /// `void setObjectState (in AString key, in nsISupports value);`
    #[inline]
    pub unsafe fn SetObjectState(&self, key: *const ::nsstring::nsAString, value: *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).SetObjectState)(self, key, value)
    }


}


/// `interface nsIHttpServerStoppedCallback : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIHttpServerStoppedCallback {
    vtable: *const nsIHttpServerStoppedCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIHttpServerStoppedCallback.
unsafe impl XpCom for nsIHttpServerStoppedCallback {
    const IID: nsIID = nsID(0x925a6d33, 0x9937, 0x4c63,
        [0xab, 0xe1, 0xa1, 0xc5, 0x6a, 0x98, 0x64, 0x55]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIHttpServerStoppedCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIHttpServerStoppedCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIHttpServerStoppedCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsIHttpServerStoppedCallback`.
    fn coerce_from(v: &nsIHttpServerStoppedCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIHttpServerStoppedCallbackCoerce for nsIHttpServerStoppedCallback {
    #[inline]
    fn coerce_from(v: &nsIHttpServerStoppedCallback) -> &Self {
        v
    }
}

impl nsIHttpServerStoppedCallback {
    /// Cast this `nsIHttpServerStoppedCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIHttpServerStoppedCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIHttpServerStoppedCallback {
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
impl<T: nsISupportsCoerce> nsIHttpServerStoppedCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIHttpServerStoppedCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIHttpServerStoppedCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIHttpServerStoppedCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onStopped (); */
    pub OnStopped: unsafe extern "system" fn (this: *const nsIHttpServerStoppedCallback) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIHttpServerStoppedCallback {

    /// ```text
    /// /**
    ///  * An interface through which a notification of the complete stopping (socket
        ///  * closure, in-flight requests all fully served and responded to) of an HTTP
    ///  * server may be received.
    ///  */
    /// /** Called when the corresponding server has been fully stopped. */
    /// ```
    ///

    /// `void onStopped ();`
    #[inline]
    pub unsafe fn OnStopped(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).OnStopped)(self, )
    }


}


/// `interface nsIHttpServerIdentity : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIHttpServerIdentity {
    vtable: *const nsIHttpServerIdentityVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIHttpServerIdentity.
unsafe impl XpCom for nsIHttpServerIdentity {
    const IID: nsIID = nsID(0xa89de175, 0xae8e, 0x4c46,
        [0x91, 0xa5, 0x0d, 0xba, 0x99, 0xbb, 0xd2, 0x84]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIHttpServerIdentity {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIHttpServerIdentity.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIHttpServerIdentityCoerce {
    /// Cheaply cast a value of this type from a `nsIHttpServerIdentity`.
    fn coerce_from(v: &nsIHttpServerIdentity) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIHttpServerIdentityCoerce for nsIHttpServerIdentity {
    #[inline]
    fn coerce_from(v: &nsIHttpServerIdentity) -> &Self {
        v
    }
}

impl nsIHttpServerIdentity {
    /// Cast this `nsIHttpServerIdentity` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIHttpServerIdentityCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIHttpServerIdentity {
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
impl<T: nsISupportsCoerce> nsIHttpServerIdentityCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIHttpServerIdentity) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIHttpServerIdentity
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIHttpServerIdentityVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute string primaryScheme; */
    pub GetPrimaryScheme: unsafe extern "system" fn (this: *const nsIHttpServerIdentity, aPrimaryScheme: *mut *const libc::c_char) -> ::nserror::nsresult,

    /* readonly attribute string primaryHost; */
    pub GetPrimaryHost: unsafe extern "system" fn (this: *const nsIHttpServerIdentity, aPrimaryHost: *mut *const libc::c_char) -> ::nserror::nsresult,

    /* readonly attribute long primaryPort; */
    pub GetPrimaryPort: unsafe extern "system" fn (this: *const nsIHttpServerIdentity, aPrimaryPort: *mut i32) -> ::nserror::nsresult,

    /* void add (in string scheme, in string host, in long port); */
    pub Add: unsafe extern "system" fn (this: *const nsIHttpServerIdentity, scheme: *const libc::c_char, host: *const libc::c_char, port: i32) -> ::nserror::nsresult,

    /* boolean remove (in string scheme, in string host, in long port); */
    pub Remove: unsafe extern "system" fn (this: *const nsIHttpServerIdentity, scheme: *const libc::c_char, host: *const libc::c_char, port: i32, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean has (in string scheme, in string host, in long port); */
    pub Has: unsafe extern "system" fn (this: *const nsIHttpServerIdentity, scheme: *const libc::c_char, host: *const libc::c_char, port: i32, _retval: *mut bool) -> ::nserror::nsresult,

    /* string getScheme (in string host, in long port); */
    pub GetScheme: unsafe extern "system" fn (this: *const nsIHttpServerIdentity, host: *const libc::c_char, port: i32, _retval: *mut *const libc::c_char) -> ::nserror::nsresult,

    /* void setPrimary (in string scheme, in string host, in long port); */
    pub SetPrimary: unsafe extern "system" fn (this: *const nsIHttpServerIdentity, scheme: *const libc::c_char, host: *const libc::c_char, port: i32) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIHttpServerIdentity {

    /// ```text
    /// /**
    ///  * Represents a set of names for a server, one of which is the primary name for
    ///  * the server and the rest of which are secondary.  By default every server will
    ///  * contain ("http", "localhost", port) and ("http", "127.0.0.1", port) as names,
    ///  * where port is what was provided to the corresponding server when started;
    ///  * however, except for their being removed when the corresponding server stops
    ///  * they have no special importance.
    ///  */
    /// /**
    ///    * The primary scheme at which the corresponding server is located, defaulting
    ///    * to 'http'.  This name will be the value of nsIHttpRequest.scheme for
    ///    * HTTP/1.0 requests.
    ///    *
    ///    * This value is always set when the corresponding server is running.  If the
    ///    * server is not running, this value is set only if it has been set to a
    ///    * non-default name using setPrimary.  In this case reading this value will
    ///    * throw NS_ERROR_NOT_INITIALIZED.
    ///    */
    /// ```
    ///

    /// `readonly attribute string primaryScheme;`
    #[inline]
    pub unsafe fn GetPrimaryScheme(&self, aPrimaryScheme: *mut *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).GetPrimaryScheme)(self, aPrimaryScheme)
    }


    /// ```text
    /// /**
    ///    * The primary name by which the corresponding server is known, defaulting to
    ///    * 'localhost'.  This name will be the value of nsIHttpRequest.host for
    ///    * HTTP/1.0 requests.
    ///    *
    ///    * This value is always set when the corresponding server is running.  If the
    ///    * server is not running, this value is set only if it has been set to a
    ///    * non-default name using setPrimary.  In this case reading this value will
    ///    * throw NS_ERROR_NOT_INITIALIZED.
    ///    */
    /// ```
    ///

    /// `readonly attribute string primaryHost;`
    #[inline]
    pub unsafe fn GetPrimaryHost(&self, aPrimaryHost: *mut *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).GetPrimaryHost)(self, aPrimaryHost)
    }


    /// ```text
    /// /**
    ///    * The primary port on which the corresponding server runs, defaulting to the
    ///    * associated server's port.  This name will be the value of
    ///    * nsIHttpRequest.port for HTTP/1.0 requests.
    ///    *
    ///    * This value is always set when the corresponding server is running.  If the
    ///    * server is not running, this value is set only if it has been set to a
    ///    * non-default name using setPrimary.  In this case reading this value will
    ///    * throw NS_ERROR_NOT_INITIALIZED.
    ///    */
    /// ```
    ///

    /// `readonly attribute long primaryPort;`
    #[inline]
    pub unsafe fn GetPrimaryPort(&self, aPrimaryPort: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetPrimaryPort)(self, aPrimaryPort)
    }


    /// ```text
    /// /**
    ///    * Adds a location at which this server may be accessed.
    ///    *
    ///    * @throws NS_ERROR_ILLEGAL_VALUE
    ///    *   if scheme or host do not match the scheme or host productions imported
    ///    *   into RFC 2616 from RFC 2396, or if port is not a valid port number
    ///    */
    /// ```
    ///

    /// `void add (in string scheme, in string host, in long port);`
    #[inline]
    pub unsafe fn Add(&self, scheme: *const libc::c_char, host: *const libc::c_char, port: i32) -> ::nserror::nsresult {
        ((*self.vtable).Add)(self, scheme, host, port)
    }


    /// ```text
    /// /**
    ///    * Removes this name from the list of names by which the corresponding server
    ///    * is known.  If name is also the primary name for the server, the primary
    ///    * name reverts to 'http://127.0.0.1' with the associated server's port.
    ///    *
    ///    * @throws NS_ERROR_ILLEGAL_VALUE
    ///    *   if scheme or host do not match the scheme or host productions imported
    ///    *   into RFC 2616 from RFC 2396, or if port is not a valid port number
    ///    * @returns
    ///    *   true if the given name was a name for this server, false otherwise
    ///    */
    /// ```
    ///

    /// `boolean remove (in string scheme, in string host, in long port);`
    #[inline]
    pub unsafe fn Remove(&self, scheme: *const libc::c_char, host: *const libc::c_char, port: i32, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).Remove)(self, scheme, host, port, _retval)
    }


    /// ```text
    /// /**
    ///    * Returns true if the given name is in this, false otherwise.
    ///    *
    ///    * @throws NS_ERROR_ILLEGAL_VALUE
    ///    *   if scheme or host do not match the scheme or host productions imported
    ///    *   into RFC 2616 from RFC 2396, or if port is not a valid port number
    ///    */
    /// ```
    ///

    /// `boolean has (in string scheme, in string host, in long port);`
    #[inline]
    pub unsafe fn Has(&self, scheme: *const libc::c_char, host: *const libc::c_char, port: i32, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).Has)(self, scheme, host, port, _retval)
    }


    /// ```text
    /// /**
    ///    * Returns the scheme for the name with the given host and port, if one is
    ///    * present; otherwise returns the empty string.
    ///    *
    ///    * @throws NS_ERROR_ILLEGAL_VALUE
    ///    *   if host does not match the host production imported into RFC 2616 from
    ///    *   RFC 2396, or if port is not a valid port number
    ///    */
    /// ```
    ///

    /// `string getScheme (in string host, in long port);`
    #[inline]
    pub unsafe fn GetScheme(&self, host: *const libc::c_char, port: i32, _retval: *mut *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).GetScheme)(self, host, port, _retval)
    }


    /// ```text
    /// /**
    ///    * Designates the given name as the primary name in this and adds it to this
    ///    * if it is not already present.
    ///    *
    ///    * @throws NS_ERROR_ILLEGAL_VALUE
    ///    *   if scheme or host do not match the scheme or host productions imported
    ///    *   into RFC 2616 from RFC 2396, or if port is not a valid port number
    ///    */
    /// ```
    ///

    /// `void setPrimary (in string scheme, in string host, in long port);`
    #[inline]
    pub unsafe fn SetPrimary(&self, scheme: *const libc::c_char, host: *const libc::c_char, port: i32) -> ::nserror::nsresult {
        ((*self.vtable).SetPrimary)(self, scheme, host, port)
    }


}


/// `interface nsIHttpRequestHandler : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIHttpRequestHandler {
    vtable: *const nsIHttpRequestHandlerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIHttpRequestHandler.
unsafe impl XpCom for nsIHttpRequestHandler {
    const IID: nsIID = nsID(0x2bbb4db7, 0xd285, 0x42b3,
        [0xa3, 0xce, 0x14, 0x2b, 0x8c, 0xc7, 0xe1, 0x39]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIHttpRequestHandler {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIHttpRequestHandler.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIHttpRequestHandlerCoerce {
    /// Cheaply cast a value of this type from a `nsIHttpRequestHandler`.
    fn coerce_from(v: &nsIHttpRequestHandler) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIHttpRequestHandlerCoerce for nsIHttpRequestHandler {
    #[inline]
    fn coerce_from(v: &nsIHttpRequestHandler) -> &Self {
        v
    }
}

impl nsIHttpRequestHandler {
    /// Cast this `nsIHttpRequestHandler` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIHttpRequestHandlerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIHttpRequestHandler {
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
impl<T: nsISupportsCoerce> nsIHttpRequestHandlerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIHttpRequestHandler) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIHttpRequestHandler
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIHttpRequestHandlerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void handle (in nsIHttpRequest request, in nsIHttpResponse response); */
    pub Handle: unsafe extern "system" fn (this: *const nsIHttpRequestHandler, request: *const nsIHttpRequest, response: *const nsIHttpResponse) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIHttpRequestHandler {

    /// ```text
    /// /**
    ///  * A representation of a handler for HTTP requests.  The handler is used by
    ///  * calling its .handle method with data for an incoming request; it is the
    ///  * handler's job to use that data as it sees fit to make the desired response.
    ///  *
    ///  * @note
    ///  *   This interface uses the [function] attribute, so you can pass a
    ///  *   script-defined function with the functionality of handle() to any
    ///  *   method which has a nsIHttpRequestHandler parameter, instead of wrapping
    ///  *   it in an otherwise empty object.
    ///  */
    /// /**
    ///    * Processes an HTTP request and initializes the passed-in response to reflect
    ///    * the correct HTTP response.
    ///    *
    ///    * If this method throws an exception, externally observable behavior depends
    ///    * upon whether is being processed asynchronously.  If such is the case, the
    ///    * output is some prefix (perhaps all, perhaps none, perhaps only some) of the
    ///    * data which would have been sent if, instead, the response had been finished
    ///    * at that point.  If no data has been written, the response has not had
    ///    * seizePower() called on it, and it is not being asynchronously created, an
    ///    * error handler will be invoked (usually 500 unless otherwise specified).
    ///    *
    ///    * Some uses of nsIHttpRequestHandler may require this method to never throw
    ///    * an exception; in the general case, however, this method may throw an
    ///    * exception (causing an HTTP 500 response to occur, if the above conditions
        ///    * are met).
    ///    *
    ///    * @param request
    ///    *   data representing an HTTP request
    ///    * @param response
    ///    *   an initially-empty response which must be modified to reflect the data
    ///    *   which should be sent as the response to the request described by metadata
    ///    */
    /// ```
    ///

    /// `void handle (in nsIHttpRequest request, in nsIHttpResponse response);`
    #[inline]
    pub unsafe fn Handle(&self, request: *const nsIHttpRequest, response: *const nsIHttpResponse) -> ::nserror::nsresult {
        ((*self.vtable).Handle)(self, request, response)
    }


}


/// `interface nsIHttpRequest : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIHttpRequest {
    vtable: *const nsIHttpRequestVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIHttpRequest.
unsafe impl XpCom for nsIHttpRequest {
    const IID: nsIID = nsID(0x978cf30e, 0xad73, 0x42ee,
        [0x8f, 0x22, 0xfe, 0x0a, 0xaf, 0x1b, 0xf5, 0xd2]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIHttpRequest {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIHttpRequest.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIHttpRequestCoerce {
    /// Cheaply cast a value of this type from a `nsIHttpRequest`.
    fn coerce_from(v: &nsIHttpRequest) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIHttpRequestCoerce for nsIHttpRequest {
    #[inline]
    fn coerce_from(v: &nsIHttpRequest) -> &Self {
        v
    }
}

impl nsIHttpRequest {
    /// Cast this `nsIHttpRequest` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIHttpRequestCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIHttpRequest {
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
impl<T: nsISupportsCoerce> nsIHttpRequestCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIHttpRequest) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIHttpRequest
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIHttpRequestVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute string method; */
    pub GetMethod: unsafe extern "system" fn (this: *const nsIHttpRequest, aMethod: *mut *const libc::c_char) -> ::nserror::nsresult,

    /* readonly attribute string scheme; */
    pub GetScheme: unsafe extern "system" fn (this: *const nsIHttpRequest, aScheme: *mut *const libc::c_char) -> ::nserror::nsresult,

    /* readonly attribute string host; */
    pub GetHost: unsafe extern "system" fn (this: *const nsIHttpRequest, aHost: *mut *const libc::c_char) -> ::nserror::nsresult,

    /* readonly attribute unsigned long port; */
    pub GetPort: unsafe extern "system" fn (this: *const nsIHttpRequest, aPort: *mut u32) -> ::nserror::nsresult,

    /* readonly attribute string path; */
    pub GetPath: unsafe extern "system" fn (this: *const nsIHttpRequest, aPath: *mut *const libc::c_char) -> ::nserror::nsresult,

    /* readonly attribute string queryString; */
    pub GetQueryString: unsafe extern "system" fn (this: *const nsIHttpRequest, aQueryString: *mut *const libc::c_char) -> ::nserror::nsresult,

    /* readonly attribute string httpVersion; */
    pub GetHttpVersion: unsafe extern "system" fn (this: *const nsIHttpRequest, aHttpVersion: *mut *const libc::c_char) -> ::nserror::nsresult,

    /* string getHeader (in string fieldName); */
    pub GetHeader: unsafe extern "system" fn (this: *const nsIHttpRequest, fieldName: *const libc::c_char, _retval: *mut *const libc::c_char) -> ::nserror::nsresult,

    /* boolean hasHeader (in string fieldName); */
    pub HasHeader: unsafe extern "system" fn (this: *const nsIHttpRequest, fieldName: *const libc::c_char, _retval: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute nsISimpleEnumerator headers; */
    pub GetHeaders: unsafe extern "system" fn (this: *const nsIHttpRequest, aHeaders: *mut*const nsISimpleEnumerator) -> ::nserror::nsresult,

    /* readonly attribute nsIInputStream bodyInputStream; */
    pub GetBodyInputStream: unsafe extern "system" fn (this: *const nsIHttpRequest, aBodyInputStream: *mut*const nsIInputStream) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIHttpRequest {

    /// ```text
    /// /**
    ///  * A representation of the data included in an HTTP request.
    ///  */
    /// /**
    ///    * The request type for this request (see RFC 2616, section 5.1.1).
    ///    */
    /// ```
    ///

    /// `readonly attribute string method;`
    #[inline]
    pub unsafe fn GetMethod(&self, aMethod: *mut *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).GetMethod)(self, aMethod)
    }


    /// ```text
    /// /**
    ///    * The scheme of the requested path, usually 'http' but might possibly be
    ///    * 'https' if some form of SSL tunneling is in use.  Note that this value
    ///    * cannot be accurately determined unless the incoming request used the
    ///    * absolute-path form of the request line; it defaults to 'http', so only
    ///    * if it is something else can you be entirely certain it's correct.
    ///    */
    /// ```
    ///

    /// `readonly attribute string scheme;`
    #[inline]
    pub unsafe fn GetScheme(&self, aScheme: *mut *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).GetScheme)(self, aScheme)
    }


    /// ```text
    /// /**
    ///    * The host of the data being requested (e.g. "localhost" for the
        ///    * http://localhost:8080/file resource).  Note that the relevant port on the
    ///    * host is specified in this.port.  This value is in the ASCII character
    ///    * encoding.
    ///    */
    /// ```
    ///

    /// `readonly attribute string host;`
    #[inline]
    pub unsafe fn GetHost(&self, aHost: *mut *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).GetHost)(self, aHost)
    }


    /// ```text
    /// /**
    ///    * The port on the server on which the request was received.
    ///    */
    /// ```
    ///

    /// `readonly attribute unsigned long port;`
    #[inline]
    pub unsafe fn GetPort(&self, aPort: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetPort)(self, aPort)
    }


    /// ```text
    /// /**
    ///    * The requested path, without any query string (e.g. "/dir/file.txt").  It is
    ///    * guaranteed to begin with a "/".  The individual components in this string
    ///    * are URL-encoded.
    ///    */
    /// ```
    ///

    /// `readonly attribute string path;`
    #[inline]
    pub unsafe fn GetPath(&self, aPath: *mut *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).GetPath)(self, aPath)
    }


    /// ```text
    /// /**
    ///    * The URL-encoded query string associated with this request, not including
    ///    * the initial "?", or "" if no query string was present.
    ///    */
    /// ```
    ///

    /// `readonly attribute string queryString;`
    #[inline]
    pub unsafe fn GetQueryString(&self, aQueryString: *mut *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).GetQueryString)(self, aQueryString)
    }


    /// ```text
    /// /**
    ///    * A string containing the HTTP version of the request (i.e., "1.1").  Leading
    ///    * zeros for either component of the version will be omitted.  (In other
        ///    * words, if the request contains the version "1.01", this attribute will be
        ///    * "1.1"; see RFC 2616, section 3.1.)
    ///    */
    /// ```
    ///

    /// `readonly attribute string httpVersion;`
    #[inline]
    pub unsafe fn GetHttpVersion(&self, aHttpVersion: *mut *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).GetHttpVersion)(self, aHttpVersion)
    }


    /// ```text
    /// /**
    ///    * Returns the value for the header in this request specified by fieldName.
    ///    *
    ///    * @param fieldName
    ///    *   the name of the field whose value is to be gotten; note that since HTTP
    ///    *   header field names are case-insensitive, this method produces equivalent
    ///    *   results for "HeAdER" and "hEADer" as fieldName
    ///    * @returns
    ///    *   The result is a string containing the individual values of the header,
    ///    *   usually separated with a comma.  The headers WWW-Authenticate,
    ///    *   Proxy-Authenticate, and Set-Cookie violate the HTTP specification,
    ///    *   however, and for these headers only the separator string is '\n'.
    ///    *
    ///    * @throws NS_ERROR_INVALID_ARG
    ///    *   if fieldName does not constitute a valid header field name
    ///    * @throws NS_ERROR_NOT_AVAILABLE
    ///    *   if the given header does not exist in this
    ///    */
    /// ```
    ///

    /// `string getHeader (in string fieldName);`
    #[inline]
    pub unsafe fn GetHeader(&self, fieldName: *const libc::c_char, _retval: *mut *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).GetHeader)(self, fieldName, _retval)
    }


    /// ```text
    /// /**
    ///    * Returns true if a header with the given field name exists in this, false
    ///    * otherwise.
    ///    *
    ///    * @param fieldName
    ///    *   the field name whose existence is to be determined in this; note that
    ///    *   since HTTP header field names are case-insensitive, this method produces
    ///    *   equivalent results for "HeAdER" and "hEADer" as fieldName
    ///    * @throws NS_ERROR_INVALID_ARG
    ///    *   if fieldName does not constitute a valid header field name
    ///    */
    /// ```
    ///

    /// `boolean hasHeader (in string fieldName);`
    #[inline]
    pub unsafe fn HasHeader(&self, fieldName: *const libc::c_char, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).HasHeader)(self, fieldName, _retval)
    }


    /// ```text
    /// /**
    ///    * An nsISimpleEnumerator of nsISupportsStrings over the names of the headers
    ///    * in this request.  The header field names in the enumerator may not
    ///    * necessarily have the same case as they do in the request itself.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsISimpleEnumerator headers;`
    #[inline]
    pub unsafe fn GetHeaders(&self, aHeaders: *mut*const nsISimpleEnumerator) -> ::nserror::nsresult {
        ((*self.vtable).GetHeaders)(self, aHeaders)
    }


    /// ```text
    /// /**
    ///    * A stream from which data appearing in the body of this request can be read.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIInputStream bodyInputStream;`
    #[inline]
    pub unsafe fn GetBodyInputStream(&self, aBodyInputStream: *mut*const nsIInputStream) -> ::nserror::nsresult {
        ((*self.vtable).GetBodyInputStream)(self, aBodyInputStream)
    }


}


/// `interface nsIHttpResponse : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIHttpResponse {
    vtable: *const nsIHttpResponseVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIHttpResponse.
unsafe impl XpCom for nsIHttpResponse {
    const IID: nsIID = nsID(0x1acd16c2, 0xdc59, 0x42fa,
        [0x91, 0x60, 0x4f, 0x26, 0xc4, 0x3c, 0x1c, 0x21]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIHttpResponse {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIHttpResponse.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIHttpResponseCoerce {
    /// Cheaply cast a value of this type from a `nsIHttpResponse`.
    fn coerce_from(v: &nsIHttpResponse) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIHttpResponseCoerce for nsIHttpResponse {
    #[inline]
    fn coerce_from(v: &nsIHttpResponse) -> &Self {
        v
    }
}

impl nsIHttpResponse {
    /// Cast this `nsIHttpResponse` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIHttpResponseCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIHttpResponse {
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
impl<T: nsISupportsCoerce> nsIHttpResponseCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIHttpResponse) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIHttpResponse
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIHttpResponseVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void setStatusLine (in string httpVersion, in unsigned short statusCode, in string description); */
    pub SetStatusLine: unsafe extern "system" fn (this: *const nsIHttpResponse, httpVersion: *const libc::c_char, statusCode: u16, description: *const libc::c_char) -> ::nserror::nsresult,

    /* void setHeader (in string name, in string value, in boolean merge); */
    pub SetHeader: unsafe extern "system" fn (this: *const nsIHttpResponse, name: *const libc::c_char, value: *const libc::c_char, merge: bool) -> ::nserror::nsresult,

    /* void setHeaderNoCheck (in string name, in string value); */
    pub SetHeaderNoCheck: unsafe extern "system" fn (this: *const nsIHttpResponse, name: *const libc::c_char, value: *const libc::c_char) -> ::nserror::nsresult,

    /* readonly attribute nsIOutputStream bodyOutputStream; */
    pub GetBodyOutputStream: unsafe extern "system" fn (this: *const nsIHttpResponse, aBodyOutputStream: *mut*const nsIOutputStream) -> ::nserror::nsresult,

    /* void write (in string data); */
    pub Write: unsafe extern "system" fn (this: *const nsIHttpResponse, data: *const libc::c_char) -> ::nserror::nsresult,

    /* void processAsync (); */
    pub ProcessAsync: unsafe extern "system" fn (this: *const nsIHttpResponse) -> ::nserror::nsresult,

    /* void seizePower (); */
    pub SeizePower: unsafe extern "system" fn (this: *const nsIHttpResponse) -> ::nserror::nsresult,

    /* void finish (); */
    pub Finish: unsafe extern "system" fn (this: *const nsIHttpResponse) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIHttpResponse {

    /// ```text
    /// /**
    ///  * Represents an HTTP response, as described in RFC 2616, section 6.
    ///  */
    /// /**
    ///    * Sets the status line for this.  If this method is never called on this, the
    ///    * status line defaults to "HTTP/", followed by the server's default HTTP
    ///    * version (e.g. "1.1"), followed by " 200 OK".
    ///    *
    ///    * @param httpVersion
    ///    *   the HTTP version of this, as a string (e.g. "1.1"); if null, the server
    ///    *   default is used
    ///    * @param code
    ///    *   the numeric HTTP status code for this
    ///    * @param description
    ///    *   a human-readable description of code; may be null if no description is
    ///    *   desired
    ///    * @throws NS_ERROR_INVALID_ARG
    ///    *   if httpVersion is not a valid HTTP version string, statusCode is greater
    ///    *   than 999, or description contains invalid characters
    ///    * @throws NS_ERROR_NOT_AVAILABLE
    ///    *   if this response is being processed asynchronously and data has been
    ///    *   written to this response's body, or if seizePower() has been called on
    ///    *   this
    ///    */
    /// ```
    ///

    /// `void setStatusLine (in string httpVersion, in unsigned short statusCode, in string description);`
    #[inline]
    pub unsafe fn SetStatusLine(&self, httpVersion: *const libc::c_char, statusCode: u16, description: *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).SetStatusLine)(self, httpVersion, statusCode, description)
    }


    /// ```text
    /// /**
    ///    * Sets the specified header in this.
    ///    *
    ///    * @param name
    ///    *   the name of the header; must match the field-name production per RFC 2616
    ///    * @param value
    ///    *   the value of the header; must match the field-value production per RFC
    ///    *   2616
    ///    * @param merge
    ///    *   when true, if the given header already exists in this, the values passed
    ///    *   to this function will be merged into the existing header, per RFC 2616
    ///    *   header semantics (except for the Set-Cookie, WWW-Authenticate, and
        ///    *   Proxy-Authenticate headers, which will treat each such merged header as
        ///    *   an additional instance of the header, for real-world compatibility
        ///    *   reasons); when false, replaces any existing header of the given name (if
        ///    *   any exists) with a new header with the specified value
    ///    * @throws NS_ERROR_INVALID_ARG
    ///    *   if name or value is not a valid header component
    ///    * @throws NS_ERROR_NOT_AVAILABLE
    ///    *   if this response is being processed asynchronously and data has been
    ///    *   written to this response's body, or if seizePower() has been called on
    ///    *   this
    ///    */
    /// ```
    ///

    /// `void setHeader (in string name, in string value, in boolean merge);`
    #[inline]
    pub unsafe fn SetHeader(&self, name: *const libc::c_char, value: *const libc::c_char, merge: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetHeader)(self, name, value, merge)
    }


    /// ```text
    /// /**
    ///    * This is used for testing our header handling, so header will be sent out
    ///    * without transformation. There can be multiple headers.
    ///    */
    /// ```
    ///

    /// `void setHeaderNoCheck (in string name, in string value);`
    #[inline]
    pub unsafe fn SetHeaderNoCheck(&self, name: *const libc::c_char, value: *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).SetHeaderNoCheck)(self, name, value)
    }


    /// ```text
    /// /**
    ///    * A stream to which data appearing in the body of this response (or in the
        ///    * totality of the response if seizePower() is called) should be written.
    ///    * After this response has been designated as being processed asynchronously,
    ///    * or after seizePower() has been called on this, subsequent writes will no
    ///    * longer be buffered and will be written to the underlying transport without
    ///    * delaying until the entire response is constructed.  Write-through may or
    ///    * may not be synchronous in the implementation, and in any case particular
    ///    * behavior may not be observable to the HTTP client as intermediate buffers
    ///    * both in the server socket and in the client may delay written data; be
    ///    * prepared for delays at any time.
    ///    *
    ///    * @throws NS_ERROR_NOT_AVAILABLE
    ///    *   if accessed after this response is fully constructed
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIOutputStream bodyOutputStream;`
    #[inline]
    pub unsafe fn GetBodyOutputStream(&self, aBodyOutputStream: *mut*const nsIOutputStream) -> ::nserror::nsresult {
        ((*self.vtable).GetBodyOutputStream)(self, aBodyOutputStream)
    }


    /// ```text
    /// /**
    ///    * Writes a string to the response's output stream.  This method is merely a
    ///    * convenient shorthand for writing the same data to bodyOutputStream
    ///    * directly.
    ///    *
    ///    * @note
    ///    *   This method is only guaranteed to work with ASCII data.
    ///    * @throws NS_ERROR_NOT_AVAILABLE
    ///    *   if called after this response has been fully constructed
    ///    */
    /// ```
    ///

    /// `void write (in string data);`
    #[inline]
    pub unsafe fn Write(&self, data: *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).Write)(self, data)
    }


    /// ```text
    /// /**
    ///    * Signals that this response is being constructed asynchronously.  Requests
    ///    * are typically completely constructed during nsIHttpRequestHandler.handle;
    ///    * however, responses which require significant resources (time, memory,
        ///    * processing) to construct can be created and sent incrementally by calling
    ///    * this method during the call to nsIHttpRequestHandler.handle.  This method
    ///    * only has this effect when called during nsIHttpRequestHandler.handle;
    ///    * behavior is undefined if it is called at a later time.  It may be called
    ///    * multiple times with no ill effect, so long as each call occurs before
    ///    * finish() is called.
    ///    *
    ///    * @throws NS_ERROR_UNEXPECTED
    ///    *   if not initially called within a nsIHttpRequestHandler.handle call or if
    ///    *   called after this response has been finished
    ///    * @throws NS_ERROR_NOT_AVAILABLE
    ///    *   if seizePower() has been called on this
    ///    */
    /// ```
    ///

    /// `void processAsync ();`
    #[inline]
    pub unsafe fn ProcessAsync(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).ProcessAsync)(self, )
    }


    /// ```text
    /// /**
    ///    * Seizes complete control of this response (and its connection) from the
    ///    * server, allowing raw and unfettered access to data being sent in the HTTP
    ///    * response.  Once this method has been called the only property which may be
    ///    * accessed without an exception being thrown is bodyOutputStream, and the
    ///    * only methods which may be accessed without an exception being thrown are
    ///    * write(), finish(), and seizePower() (which may be called multiple times
        ///    * without ill effect so long as all calls are otherwise allowed).
    ///    *
    ///    * After a successful call, all data subsequently written to the body of this
    ///    * response is written directly to the corresponding connection.  (Previously-
        ///    * written data is silently discarded.)  No status line or headers are sent
    ///    * before doing so; if the response handler wishes to write such data, it must
    ///    * do so manually.  Data generation completes only when finish() is called; it
    ///    * is not enough to simply call close() on bodyOutputStream.
    ///    *
    ///    * @throws NS_ERROR_NOT_AVAILABLE
    ///    *   if processAsync() has been called on this
    ///    * @throws NS_ERROR_UNEXPECTED
    ///    *   if finish() has been called on this
    ///    */
    /// ```
    ///

    /// `void seizePower ();`
    #[inline]
    pub unsafe fn SeizePower(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).SeizePower)(self, )
    }


    /// ```text
    /// /**
    ///    * Signals that construction of this response is complete and that it may be
    ///    * sent over the network to the client, or if seizePower() has been called
    ///    * signals that all data has been written and that the underlying connection
    ///    * may be closed.  This method may only be called after processAsync() or
    ///    * seizePower() has been called.  This method is idempotent.
    ///    *
    ///    * @throws NS_ERROR_UNEXPECTED
    ///    *   if processAsync() or seizePower() has not already been properly called
    ///    */
    /// ```
    ///

    /// `void finish ();`
    #[inline]
    pub unsafe fn Finish(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Finish)(self, )
    }


}


