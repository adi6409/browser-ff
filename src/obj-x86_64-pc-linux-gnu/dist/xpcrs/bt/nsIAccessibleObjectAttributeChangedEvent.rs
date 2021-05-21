//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/accessible/interfaces/nsIAccessibleObjectAttributeChangedEvent.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAccessibleObjectAttributeChangedEvent",
            base: Some("nsIAccessibleEvent"),
            methods: Ok(&[
                    /* readonly attribute AString changedAttribute; */
                    Method {
                        name: "GetChangedAttribute",
                        params: &[Param { name: "aChangedAttribute", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

