//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/push/nsIPushErrorReporter.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPushErrorReporter",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void reportDeliveryError (in AString messageId, [optional] in uint16_t reason); */
                    Method {
                        name: "ReportDeliveryError",
                        params: &[Param { name: "messageId", ty: "*const ::nsstring::nsAString" }, Param { name: "reason", ty: "uint16_t" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

