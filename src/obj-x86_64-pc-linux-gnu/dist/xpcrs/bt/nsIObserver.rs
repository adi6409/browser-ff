//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/ds/nsIObserver.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIObserver",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void observe (in nsISupports aSubject, in string aTopic, in wstring aData); */
                    Method {
                        name: "Observe",
                        params: &[Param { name: "aSubject", ty: "*const nsISupports" }, Param { name: "aTopic", ty: "*const libc::c_char" }, Param { name: "aData", ty: "*const i16" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

