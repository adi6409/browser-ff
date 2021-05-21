//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/power/nsIDOMWakeLockListener.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMMozWakeLockListener",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void callback (in AString aTopic, in AString aState); */
                    Method {
                        name: "Callback",
                        params: &[Param { name: "aTopic", ty: "*const ::nsstring::nsAString" }, Param { name: "aState", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

