//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/base/nsIPermissionDelegateHandler.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPermissionDelegateHandler",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* boolean maybeUnsafePermissionDelegate (in Array<ACString> aTypes); */
                    Method {
                        name: "MaybeUnsafePermissionDelegate",
                        params: &[Param { name: "aTypes", ty: "*const thin_vec::ThinVec<::nsstring::nsCString>" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean permissionDelegateFPEnabled; */
                    Method {
                        name: "GetPermissionDelegateFPEnabled",
                        params: &[Param { name: "aPermissionDelegateFPEnabled", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

