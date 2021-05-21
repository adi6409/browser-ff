//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/manager/ssl/nsIPKCS11Slot.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPKCS11Slot",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* [must_use] readonly attribute AUTF8String name; */
                    Method {
                        name: "GetName",
                        params: &[Param { name: "aName", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] readonly attribute AUTF8String desc; */
                    Method {
                        name: "GetDesc",
                        params: &[Param { name: "aDesc", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] readonly attribute AUTF8String manID; */
                    Method {
                        name: "GetManID",
                        params: &[Param { name: "aManID", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] readonly attribute AUTF8String HWVersion; */
                    Method {
                        name: "GetHWVersion",
                        params: &[Param { name: "aHWVersion", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] readonly attribute AUTF8String FWVersion; */
                    Method {
                        name: "GetFWVersion",
                        params: &[Param { name: "aFWVersion", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] readonly attribute unsigned long status; */
                    Method {
                        name: "GetStatus",
                        params: &[Param { name: "aStatus", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] nsIPK11Token getToken (); */
                    Method {
                        name: "GetToken",
                        params: &[Param { name: "_retval", ty: "*mut*const nsIPK11Token" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] readonly attribute AUTF8String tokenName; */
                    Method {
                        name: "GetTokenName",
                        params: &[Param { name: "aTokenName", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

