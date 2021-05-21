//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIHttpAuthenticatorCallback.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIHttpAuthenticatorCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onCredsGenerated (in string aCreds, in unsigned long aFlags, in nsresult aResult, in nsISupports aSessionsState, in nsISupports aContinuationState); */
                    Method {
                        name: "OnCredsGenerated",
                        params: &[Param { name: "aCreds", ty: "*const libc::c_char" }, Param { name: "aFlags", ty: "u32" }, Param { name: "aResult", ty: "::nserror::nsresult" }, Param { name: "aSessionsState", ty: "*const nsISupports" }, Param { name: "aContinuationState", ty: "*const nsISupports" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

