//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/uriloader/base/nsIURIContentListener.idl
//


/// `interface nsIURIContentListener : nsISupports`
///

/// ```text
/// /**
///  * nsIURIContentListener is an interface used by components which
///  * want to know (and have a chance to handle) a particular content type.
///  * Typical usage scenarios will include running applications which register
///  * a nsIURIContentListener for each of its content windows with the uri
///  * dispatcher service.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIURIContentListener {
    vtable: *const nsIURIContentListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIURIContentListener.
unsafe impl XpCom for nsIURIContentListener {
    const IID: nsIID = nsID(0x10a28f38, 0x32e8, 0x4c63,
        [0x8a, 0xa1, 0x12, 0xea, 0xae, 0xbc, 0x36, 0x9a]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIURIContentListener {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIURIContentListener.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIURIContentListenerCoerce {
    /// Cheaply cast a value of this type from a `nsIURIContentListener`.
    fn coerce_from(v: &nsIURIContentListener) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIURIContentListenerCoerce for nsIURIContentListener {
    #[inline]
    fn coerce_from(v: &nsIURIContentListener) -> &Self {
        v
    }
}

impl nsIURIContentListener {
    /// Cast this `nsIURIContentListener` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIURIContentListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIURIContentListener {
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
impl<T: nsISupportsCoerce> nsIURIContentListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIURIContentListener) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIURIContentListener
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIURIContentListenerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* boolean doContent (in ACString aContentType, in boolean aIsContentPreferred, in nsIRequest aRequest, out nsIStreamListener aContentHandler); */
    pub DoContent: unsafe extern "system" fn (this: *const nsIURIContentListener, aContentType: *const ::nsstring::nsACString, aIsContentPreferred: bool, aRequest: *const nsIRequest, aContentHandler: *mut*const nsIStreamListener, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean isPreferred (in string aContentType, out string aDesiredContentType); */
    pub IsPreferred: unsafe extern "system" fn (this: *const nsIURIContentListener, aContentType: *const libc::c_char, aDesiredContentType: *mut *const libc::c_char, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean canHandleContent (in string aContentType, in boolean aIsContentPreferred, out string aDesiredContentType); */
    pub CanHandleContent: unsafe extern "system" fn (this: *const nsIURIContentListener, aContentType: *const libc::c_char, aIsContentPreferred: bool, aDesiredContentType: *mut *const libc::c_char, _retval: *mut bool) -> ::nserror::nsresult,

    /* attribute nsISupports loadCookie; */
    pub GetLoadCookie: unsafe extern "system" fn (this: *const nsIURIContentListener, aLoadCookie: *mut *const nsISupports) -> ::nserror::nsresult,

    /* attribute nsISupports loadCookie; */
    pub SetLoadCookie: unsafe extern "system" fn (this: *const nsIURIContentListener, aLoadCookie: *const nsISupports) -> ::nserror::nsresult,

    /* attribute nsIURIContentListener parentContentListener; */
    pub GetParentContentListener: unsafe extern "system" fn (this: *const nsIURIContentListener, aParentContentListener: *mut *const nsIURIContentListener) -> ::nserror::nsresult,

    /* attribute nsIURIContentListener parentContentListener; */
    pub SetParentContentListener: unsafe extern "system" fn (this: *const nsIURIContentListener, aParentContentListener: *const nsIURIContentListener) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIURIContentListener {

    /// ```text
    /// /**
    ///   * Notifies the content listener to hook up an nsIStreamListener capable of
    ///   * consuming the data stream.
    ///   *
    ///   * @param aContentType         Content type of the data.
    ///   * @param aIsContentPreferred  Indicates whether the content should be
    ///   *                             preferred by this listener.
    ///   * @param aRequest             Request that is providing the data.
    ///   * @param aContentHandler      nsIStreamListener that will consume the data.
    ///   *                             This should be set to <code>nullptr</code> if
    ///   *                             this content listener can't handle the content
    ///   *                             type; in this case, doContent should also fail
    ///   *                             (i.e., return failure nsresult).
    ///   *
    ///   * @return                     <code>true</code> if the load should
    ///   *                             be aborted and consumer wants to
    ///   *                             handle the load completely by itself.  This
    ///   *                             causes the URI Loader do nothing else...
    ///   *                             <code>false</code> if the URI Loader should
    ///   *                             continue handling the load and call the
    ///   *                             returned streamlistener's methods.
    ///   */
    /// ```
    ///

    /// `boolean doContent (in ACString aContentType, in boolean aIsContentPreferred, in nsIRequest aRequest, out nsIStreamListener aContentHandler);`
    #[inline]
    pub unsafe fn DoContent(&self, aContentType: *const ::nsstring::nsACString, aIsContentPreferred: bool, aRequest: *const nsIRequest, aContentHandler: *mut*const nsIStreamListener, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).DoContent)(self, aContentType, aIsContentPreferred, aRequest, aContentHandler, _retval)
    }


    /// ```text
    /// /**
    ///   * When given a uri to dispatch, if the URI is specified as 'preferred
    ///   * content' then the uri loader tries to find a preferred content handler
    ///   * for the content type. The thought is that many content listeners may
    ///   * be able to handle the same content type if they have to. i.e. the mail
    ///   * content window can handle text/html just like a browser window content
    ///   * listener. However, if the user clicks on a link with text/html content,
    ///   * then the browser window should handle that content and not the mail
    ///   * window where the user may have clicked the link.  This is the difference
    ///   * between isPreferred and canHandleContent.
    ///   *
    ///   * @param aContentType         Content type of the data.
    ///   * @param aDesiredContentType  Indicates that aContentType must be converted
    ///   *                             to aDesiredContentType before processing the
    ///   *                             data.  This causes a stream converted to be
    ///   *                             inserted into the nsIStreamListener chain.
    ///   *                             This argument can be <code>nullptr</code> if
    ///   *                             the content should be consumed directly as
    ///   *                             aContentType.
    ///   *
    ///   * @return                     <code>true</code> if this is a preferred
    ///   *                             content handler for aContentType;
    ///   *                             <code>false<code> otherwise.
    ///   */
    /// ```
    ///

    /// `boolean isPreferred (in string aContentType, out string aDesiredContentType);`
    #[inline]
    pub unsafe fn IsPreferred(&self, aContentType: *const libc::c_char, aDesiredContentType: *mut *const libc::c_char, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsPreferred)(self, aContentType, aDesiredContentType, _retval)
    }


    /// ```text
    /// /**
    ///   * When given a uri to dispatch, if the URI is not specified as 'preferred
    ///   * content' then the uri loader calls canHandleContent to see if the content
    ///   * listener is capable of handling the content.
    ///   *
    ///   * @param aContentType         Content type of the data.
    ///   * @param aIsContentPreferred  Indicates whether the content should be
    ///   *                             preferred by this listener.
    ///   * @param aDesiredContentType  Indicates that aContentType must be converted
    ///   *                             to aDesiredContentType before processing the
    ///   *                             data.  This causes a stream converted to be
    ///   *                             inserted into the nsIStreamListener chain.
    ///   *                             This argument can be <code>nullptr</code> if
    ///   *                             the content should be consumed directly as
    ///   *                             aContentType.
    ///   *
    ///   * @return                     <code>true</code> if the data can be consumed.
    ///   *                             <code>false</code> otherwise.
    ///   *
    ///   * Note: I really envision canHandleContent as a method implemented
    ///   * by the docshell as the implementation is generic to all doc
    ///   * shells. The isPreferred decision is a decision made by a top level
    ///   * application content listener that sits at the top of the docshell
    ///   * hierarchy.
    ///   */
    /// ```
    ///

    /// `boolean canHandleContent (in string aContentType, in boolean aIsContentPreferred, out string aDesiredContentType);`
    #[inline]
    pub unsafe fn CanHandleContent(&self, aContentType: *const libc::c_char, aIsContentPreferred: bool, aDesiredContentType: *mut *const libc::c_char, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).CanHandleContent)(self, aContentType, aIsContentPreferred, aDesiredContentType, _retval)
    }


    /// ```text
    /// /**
    ///   * The load context associated with a particular content listener.
    ///   * The URI Loader stores and accesses this value as needed.
    ///   */
    /// ```
    ///

    /// `attribute nsISupports loadCookie;`
    #[inline]
    pub unsafe fn GetLoadCookie(&self, aLoadCookie: *mut *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).GetLoadCookie)(self, aLoadCookie)
    }


    /// ```text
    /// /**
    ///   * The load context associated with a particular content listener.
    ///   * The URI Loader stores and accesses this value as needed.
    ///   */
    /// ```
    ///

    /// `attribute nsISupports loadCookie;`
    #[inline]
    pub unsafe fn SetLoadCookie(&self, aLoadCookie: *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).SetLoadCookie)(self, aLoadCookie)
    }


    /// ```text
    /// /**
    ///   * The parent content listener if this particular listener is part of a chain
    ///   * of content listeners (i.e. a docshell!)
    ///   *
    ///   * @note If this attribute is set to an object that implements
    ///   *       nsISupportsWeakReference, the implementation should get the
    ///   *       nsIWeakReference and hold that.  Otherwise, the implementation
    ///   *       should not refcount this interface; it should assume that a non
    ///   *       null value is always valid.  In that case, the caller is
    ///   *       responsible for explicitly setting this value back to null if the
    ///   *       parent content listener is destroyed.
    ///   */
    /// ```
    ///

    /// `attribute nsIURIContentListener parentContentListener;`
    #[inline]
    pub unsafe fn GetParentContentListener(&self, aParentContentListener: *mut *const nsIURIContentListener) -> ::nserror::nsresult {
        ((*self.vtable).GetParentContentListener)(self, aParentContentListener)
    }


    /// ```text
    /// /**
    ///   * The parent content listener if this particular listener is part of a chain
    ///   * of content listeners (i.e. a docshell!)
    ///   *
    ///   * @note If this attribute is set to an object that implements
    ///   *       nsISupportsWeakReference, the implementation should get the
    ///   *       nsIWeakReference and hold that.  Otherwise, the implementation
    ///   *       should not refcount this interface; it should assume that a non
    ///   *       null value is always valid.  In that case, the caller is
    ///   *       responsible for explicitly setting this value back to null if the
    ///   *       parent content listener is destroyed.
    ///   */
    /// ```
    ///

    /// `attribute nsIURIContentListener parentContentListener;`
    #[inline]
    pub unsafe fn SetParentContentListener(&self, aParentContentListener: *const nsIURIContentListener) -> ::nserror::nsresult {
        ((*self.vtable).SetParentContentListener)(self, aParentContentListener)
    }


}


