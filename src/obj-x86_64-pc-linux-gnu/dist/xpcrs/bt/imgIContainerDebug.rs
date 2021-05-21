//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/image/imgIContainerDebug.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "imgIContainerDebug",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute uint32_t framesNotified; */
                    Method {
                        name: "GetFramesNotified",
                        params: &[Param { name: "aFramesNotified", ty: "*mut uint32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

