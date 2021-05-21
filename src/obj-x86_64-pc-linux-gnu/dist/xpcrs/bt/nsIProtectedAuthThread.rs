//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/manager/ssl/nsIProtectedAuthThread.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIProtectedAuthThread",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* [must_use] void login (in nsIObserver observer); */
                    Method {
                        name: "Login",
                        params: &[Param { name: "observer", ty: "*const nsIObserver" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] readonly attribute nsIPKCS11Slot slot; */
                    Method {
                        name: "GetSlot",
                        params: &[Param { name: "aSlot", ty: "*mut *const nsIPKCS11Slot" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] AString getTokenName (); */
                    Method {
                        name: "GetTokenName",
                        params: &[Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

