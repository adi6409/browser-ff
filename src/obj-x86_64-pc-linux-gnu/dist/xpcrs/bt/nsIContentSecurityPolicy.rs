//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/security/nsIContentSecurityPolicy.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIContentSecurityPolicy",
            base: Some("nsISerializable"),
            methods: Err("nostdcall is unsupported"),
        },

        Interface {
            name: "nsICSPEventListener",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onCSPViolationEvent (in AString aJSON); */
                    Method {
                        name: "OnCSPViolationEvent",
                        params: &[Param { name: "aJSON", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

