//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/presentation/interfaces/nsIPresentationLocalDevice.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPresentationLocalDevice",
            base: Some("nsIPresentationDevice"),
            methods: Ok(&[
                    /* readonly attribute AUTF8String windowId; */
                    Method {
                        name: "GetWindowId",
                        params: &[Param { name: "aWindowId", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

