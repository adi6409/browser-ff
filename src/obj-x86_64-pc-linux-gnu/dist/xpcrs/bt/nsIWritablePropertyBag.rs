//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/ds/nsIWritablePropertyBag.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIWritablePropertyBag",
            base: Some("nsIPropertyBag"),
            methods: Ok(&[
                    /* void setProperty (in AString name, in nsIVariant value); */
                    Method {
                        name: "SetProperty",
                        params: &[Param { name: "name", ty: "*const ::nsstring::nsAString" }, Param { name: "value", ty: "*const nsIVariant" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void deleteProperty (in AString name); */
                    Method {
                        name: "DeleteProperty",
                        params: &[Param { name: "name", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

