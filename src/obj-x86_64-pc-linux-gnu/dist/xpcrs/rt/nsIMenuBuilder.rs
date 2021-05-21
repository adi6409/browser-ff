//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/html/nsIMenuBuilder.idl
//


/// `interface nsIMenuBuilder : nsISupports`
///

/// ```text
/// /**
///  * An interface used to construct native toolbar or context menus from <menu>
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIMenuBuilder {
    vtable: *const nsIMenuBuilderVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIMenuBuilder.
unsafe impl XpCom for nsIMenuBuilder {
    const IID: nsIID = nsID(0x93f4a48f, 0xd043, 0x4f45,
        [0x97, 0xfd, 0x97, 0x71, 0xea, 0x1a, 0xf9, 0x76]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIMenuBuilder {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIMenuBuilder.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIMenuBuilderCoerce {
    /// Cheaply cast a value of this type from a `nsIMenuBuilder`.
    fn coerce_from(v: &nsIMenuBuilder) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIMenuBuilderCoerce for nsIMenuBuilder {
    #[inline]
    fn coerce_from(v: &nsIMenuBuilder) -> &Self {
        v
    }
}

impl nsIMenuBuilder {
    /// Cast this `nsIMenuBuilder` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIMenuBuilderCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIMenuBuilder {
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
impl<T: nsISupportsCoerce> nsIMenuBuilderCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIMenuBuilder) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIMenuBuilder
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIMenuBuilderVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void openContainer (in AString aLabel); */
    pub OpenContainer: unsafe extern "system" fn (this: *const nsIMenuBuilder, aLabel: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void addItemFor (in Element aElement, in boolean aCanLoadIcon); */
    pub AddItemFor: unsafe extern "system" fn (this: *const nsIMenuBuilder, aElement: *const libc::c_void, aCanLoadIcon: bool) -> ::nserror::nsresult,

    /* void addSeparator (); */
    pub AddSeparator: unsafe extern "system" fn (this: *const nsIMenuBuilder) -> ::nserror::nsresult,

    /* void undoAddSeparator (); */
    pub UndoAddSeparator: unsafe extern "system" fn (this: *const nsIMenuBuilder) -> ::nserror::nsresult,

    /* void closeContainer (); */
    pub CloseContainer: unsafe extern "system" fn (this: *const nsIMenuBuilder) -> ::nserror::nsresult,

    /* AString toJSONString (); */
    pub ToJSONString: unsafe extern "system" fn (this: *const nsIMenuBuilder, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void click (in AString aGeneratedItemId); */
    pub Click: unsafe extern "system" fn (this: *const nsIMenuBuilder, aGeneratedItemId: *const ::nsstring::nsAString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIMenuBuilder {

    /// ```text
    /// /**
    ///    * Create the top level menu or a submenu. The implementation should create
    ///    * a new context for this menu, so all subsequent methods will add new items
    ///    * to this newly created menu.
    ///    */
    /// ```
    ///

    /// `void openContainer (in AString aLabel);`
    #[inline]
    pub unsafe fn OpenContainer(&self, aLabel: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).OpenContainer)(self, aLabel)
    }


    /// ```text
    /// /**
    ///    * Add a new menu item. All menu item details can be obtained from
    ///    * the element. This method is not called for hidden elements or elements
    ///    * with no or empty label. The icon should be loaded only if aCanLoadIcon
    ///    * is true.
    ///    */
    /// ```
    ///

    /// `void addItemFor (in Element aElement, in boolean aCanLoadIcon);`
    #[inline]
    pub unsafe fn AddItemFor(&self, aElement: *const libc::c_void, aCanLoadIcon: bool) -> ::nserror::nsresult {
        ((*self.vtable).AddItemFor)(self, aElement, aCanLoadIcon)
    }


    /// ```text
    /// /**
    ///    * Create a new separator.
    ///    */
    /// ```
    ///

    /// `void addSeparator ();`
    #[inline]
    pub unsafe fn AddSeparator(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).AddSeparator)(self, )
    }


    /// ```text
    /// /**
    ///    * Remove last added separator.
    ///    * Sometimes it's needed to remove last added separator, otherwise it's not
    ///    * possible to implement the postprocessing in one pass.
    ///    * See http://www.whatwg.org/specs/web-apps/current-work/multipage/interactive-elements.html#building-menus-and-toolbars
    ///    */
    /// ```
    ///

    /// `void undoAddSeparator ();`
    #[inline]
    pub unsafe fn UndoAddSeparator(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).UndoAddSeparator)(self, )
    }


    /// ```text
    /// /**
    ///    * Set the context to the parent menu.
    ///    */
    /// ```
    ///

    /// `void closeContainer ();`
    #[inline]
    pub unsafe fn CloseContainer(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).CloseContainer)(self, )
    }


    /// ```text
    /// /**
    ///    * Returns a JSON string representing the menu hierarchy. For a context menu,
    ///    * it will be of the form:
    ///    *  {
        ///    *    type: "menu",
        ///    *    children: [
            ///    *      {
                ///    *        type: "menuitem",
                ///    *        label: "label",
                ///    *        icon: "image.png"
                ///    *      },
            ///    *      {
                ///    *        type: "separator",
                ///    *      },
            ///    *    ];
        ///    */
        /// ```
        ///

        /// `AString toJSONString ();`
        #[inline]
        pub unsafe fn ToJSONString(&self, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
            ((*self.vtable).ToJSONString)(self, _retval)
        }


        /// ```text
        /// /**
        ///    * Invoke the action of the menuitem with assigned id aGeneratedItemId.
        ///    *
        ///    * @param aGeneratedItemId the menuitem id
        ///    */
        /// ```
        ///

        /// `void click (in AString aGeneratedItemId);`
        #[inline]
        pub unsafe fn Click(&self, aGeneratedItemId: *const ::nsstring::nsAString) -> ::nserror::nsresult {
            ((*self.vtable).Click)(self, aGeneratedItemId)
        }


    }


