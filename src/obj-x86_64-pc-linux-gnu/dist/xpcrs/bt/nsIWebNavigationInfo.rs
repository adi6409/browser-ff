//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/docshell/base/nsIWebNavigationInfo.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIWebNavigationInfo",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* unsigned long isTypeSupported (in ACString aType, in nsIWebNavigation aWebNav); */
                    Method {
                        name: "IsTypeSupported",
                        params: &[Param { name: "aType", ty: "*const ::nsstring::nsACString" }, Param { name: "aWebNav", ty: "*const nsIWebNavigation" }, Param { name: "_retval", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

