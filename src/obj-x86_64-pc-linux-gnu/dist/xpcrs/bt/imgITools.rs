//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/image/imgITools.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "imgITools",
            base: Some("nsISupports"),
            methods: Err("specialtype jsval unsupported"),
        },

        Interface {
            name: "imgIContainerCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onImageReady (in imgIContainer aImage, in nsresult aStatus); */
                    Method {
                        name: "OnImageReady",
                        params: &[Param { name: "aImage", ty: "*const imgIContainer" }, Param { name: "aStatus", ty: "::nserror::nsresult" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

