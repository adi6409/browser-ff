//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/manager/ssl/nsIPKCS11Module.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPKCS11Module",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* [must_use] readonly attribute AUTF8String name; */
                    Method {
                        name: "GetName",
                        params: &[Param { name: "aName", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] readonly attribute AUTF8String libName; */
                    Method {
                        name: "GetLibName",
                        params: &[Param { name: "aLibName", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] nsISimpleEnumerator listSlots (); */
                    Method {
                        name: "ListSlots",
                        params: &[Param { name: "_retval", ty: "*mut*const nsISimpleEnumerator" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

