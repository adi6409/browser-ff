//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/layout/xul/tree/nsITreeSelection.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsITreeSelection",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* attribute XULTreeElement tree; */
                    Method {
                        name: "GetTree",
                        params: &[Param { name: "aTree", ty: "*mut *const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetTree",
                        params: &[Param { name: "aTree", ty: "*const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean single; */
                    Method {
                        name: "GetSingle",
                        params: &[Param { name: "aSingle", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute long count; */
                    Method {
                        name: "GetCount",
                        params: &[Param { name: "aCount", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean isSelected (in long index); */
                    Method {
                        name: "IsSelected",
                        params: &[Param { name: "index", ty: "i32" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void select (in long index); */
                    Method {
                        name: "Select",
                        params: &[Param { name: "index", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void timedSelect (in long index, in long delay); */
                    Method {
                        name: "TimedSelect",
                        params: &[Param { name: "index", ty: "i32" }, Param { name: "delay", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void toggleSelect (in long index); */
                    Method {
                        name: "ToggleSelect",
                        params: &[Param { name: "index", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void rangedSelect (in long startIndex, in long endIndex, in boolean augment); */
                    Method {
                        name: "RangedSelect",
                        params: &[Param { name: "startIndex", ty: "i32" }, Param { name: "endIndex", ty: "i32" }, Param { name: "augment", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void clearRange (in long startIndex, in long endIndex); */
                    Method {
                        name: "ClearRange",
                        params: &[Param { name: "startIndex", ty: "i32" }, Param { name: "endIndex", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void clearSelection (); */
                    Method {
                        name: "ClearSelection",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void selectAll (); */
                    Method {
                        name: "SelectAll",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* long getRangeCount (); */
                    Method {
                        name: "GetRangeCount",
                        params: &[Param { name: "_retval", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void getRangeAt (in long i, out long min, out long max); */
                    Method {
                        name: "GetRangeAt",
                        params: &[Param { name: "i", ty: "i32" }, Param { name: "min", ty: "*mut i32" }, Param { name: "max", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void invalidateSelection (); */
                    Method {
                        name: "InvalidateSelection",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void adjustSelection (in long index, in long count); */
                    Method {
                        name: "AdjustSelection",
                        params: &[Param { name: "index", ty: "i32" }, Param { name: "count", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute boolean selectEventsSuppressed; */
                    Method {
                        name: "GetSelectEventsSuppressed",
                        params: &[Param { name: "aSelectEventsSuppressed", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetSelectEventsSuppressed",
                        params: &[Param { name: "aSelectEventsSuppressed", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute long currentIndex; */
                    Method {
                        name: "GetCurrentIndex",
                        params: &[Param { name: "aCurrentIndex", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetCurrentIndex",
                        params: &[Param { name: "aCurrentIndex", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute long shiftSelectPivot; */
                    Method {
                        name: "GetShiftSelectPivot",
                        params: &[Param { name: "aShiftSelectPivot", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsINativeTreeSelection",
            base: Some("nsITreeSelection"),
            methods: Ok(&[
                    /* [noscript] void ensureNative (); */
                    Method {
                        name: "EnsureNative",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

