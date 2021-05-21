//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/modules/libjar/nsIJARChannel.idl
//


/// `interface nsIJARChannel : nsIChannel`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIJARChannel {
    vtable: *const nsIJARChannelVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIJARChannel.
unsafe impl XpCom for nsIJARChannel {
    const IID: nsIID = nsID(0xe72b179b, 0xd5df, 0x4d87,
        [0xb5, 0xde, 0xfd, 0x73, 0xa6, 0x5c, 0x60, 0xf6]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIJARChannel {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIJARChannel.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIJARChannelCoerce {
    /// Cheaply cast a value of this type from a `nsIJARChannel`.
    fn coerce_from(v: &nsIJARChannel) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIJARChannelCoerce for nsIJARChannel {
    #[inline]
    fn coerce_from(v: &nsIJARChannel) -> &Self {
        v
    }
}

impl nsIJARChannel {
    /// Cast this `nsIJARChannel` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIJARChannelCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIJARChannel {
    type Target = nsIChannel;
    #[inline]
    fn deref(&self) -> &nsIChannel {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIChannelCoerce> nsIJARChannelCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIJARChannel) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIJARChannel
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIJARChannelVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIChannelVTable,

    /* attribute nsIFile jarFile; */
    pub GetJarFile: unsafe extern "system" fn (this: *const nsIJARChannel, aJarFile: *mut*const nsIFile) -> ::nserror::nsresult,

    /* attribute nsIFile jarFile; */
    pub SetJarFile: unsafe extern "system" fn (this: *const nsIJARChannel, aJarFile: *const nsIFile) -> ::nserror::nsresult,

    /* readonly attribute nsIZipEntry zipEntry; */
    pub GetZipEntry: unsafe extern "system" fn (this: *const nsIJARChannel, aZipEntry: *mut*const nsIZipEntry) -> ::nserror::nsresult,

    /* boolean ensureCached (); */
    pub EnsureCached: unsafe extern "system" fn (this: *const nsIJARChannel, _retval: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIJARChannel {

    /// ```text
    /// /**
    ///      * Returns the JAR file.  May be null if the jar is remote.
    ///      * Setting the JAR file is optional and overrides the JAR
    ///      * file used for local file JARs. Setting the JAR file after
    ///      * the channel has been opened is not permitted.
    ///      */
    /// ```
    ///

    /// `attribute nsIFile jarFile;`
    #[inline]
    pub unsafe fn GetJarFile(&self, aJarFile: *mut*const nsIFile) -> ::nserror::nsresult {
        ((*self.vtable).GetJarFile)(self, aJarFile)
    }


    /// ```text
    /// /**
    ///      * Returns the JAR file.  May be null if the jar is remote.
    ///      * Setting the JAR file is optional and overrides the JAR
    ///      * file used for local file JARs. Setting the JAR file after
    ///      * the channel has been opened is not permitted.
    ///      */
    /// ```
    ///

    /// `attribute nsIFile jarFile;`
    #[inline]
    pub unsafe fn SetJarFile(&self, aJarFile: *const nsIFile) -> ::nserror::nsresult {
        ((*self.vtable).SetJarFile)(self, aJarFile)
    }


    /// ```text
    /// /**
    ///      * Returns the zip entry if the file is synchronously accessible.
    ///      * This will work even without opening the channel.
    ///      */
    /// ```
    ///

    /// `readonly attribute nsIZipEntry zipEntry;`
    #[inline]
    pub unsafe fn GetZipEntry(&self, aZipEntry: *mut*const nsIZipEntry) -> ::nserror::nsresult {
        ((*self.vtable).GetZipEntry)(self, aZipEntry)
    }


    /// ```text
    /// /**
    ///      * If the JAR file is cached in the JAR cache, returns true and
    ///      * holds a reference to the cached zip reader to be used when
    ///      * the channel is read from, ensuring the cached reader will be used.
    ///      * For a successful read from the cached reader, close() should not
    ///      * be called on the reader--per nsIZipReader::getZip() documentation.
    ///      * Returns false if the JAR file is not cached. Calling this method
    ///      * after the channel has been opened is not permitted.
    ///      */
    /// ```
    ///

    /// `boolean ensureCached ();`
    #[inline]
    pub unsafe fn EnsureCached(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).EnsureCached)(self, _retval)
    }


}


