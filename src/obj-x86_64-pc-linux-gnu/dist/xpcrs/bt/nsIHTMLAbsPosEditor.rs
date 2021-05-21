//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/editor/nsIHTMLAbsPosEditor.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIHTMLAbsPosEditor",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* [can_run_script] attribute boolean absolutePositioningEnabled; */
                    Method {
                        name: "GetAbsolutePositioningEnabled",
                        params: &[Param { name: "aAbsolutePositioningEnabled", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetAbsolutePositioningEnabled",
                        params: &[Param { name: "aAbsolutePositioningEnabled", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute boolean snapToGridEnabled; */
                    Method {
                        name: "GetSnapToGridEnabled",
                        params: &[Param { name: "aSnapToGridEnabled", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetSnapToGridEnabled",
                        params: &[Param { name: "aSnapToGridEnabled", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute unsigned long gridSize; */
                    Method {
                        name: "GetGridSize",
                        params: &[Param { name: "aGridSize", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetGridSize",
                        params: &[Param { name: "aGridSize", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [can_run_script] void refreshGrabber (); */
                    Method {
                        name: "RefreshGrabber",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

