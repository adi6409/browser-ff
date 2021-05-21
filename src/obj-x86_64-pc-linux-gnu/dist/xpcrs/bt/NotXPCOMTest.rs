//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/tests/NotXPCOMTest.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIScriptableOK",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void method1 (); */
                    Method {
                        name: "Method1",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIScriptableWithNotXPCOM",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* [notxpcom] void method2 (); */
                    Method {
                        name: "Method2",
                        params: &[],
                        ret: "libc::c_void",
                    },

                    ]),
        },

        ]; D}

