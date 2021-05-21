//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/base/nsIQueryContentEventResult.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIQueryContentEventResult",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute unsigned long offset; */
                    Method {
                        name: "GetOffset",
                        params: &[Param { name: "aOffset", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long tentativeCaretOffset; */
                    Method {
                        name: "GetTentativeCaretOffset",
                        params: &[Param { name: "aTentativeCaretOffset", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean reversed; */
                    Method {
                        name: "GetReversed",
                        params: &[Param { name: "aReversed", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute long left; */
                    Method {
                        name: "GetLeft",
                        params: &[Param { name: "aLeft", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute long top; */
                    Method {
                        name: "GetTop",
                        params: &[Param { name: "aTop", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute long width; */
                    Method {
                        name: "GetWidth",
                        params: &[Param { name: "aWidth", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute long height; */
                    Method {
                        name: "GetHeight",
                        params: &[Param { name: "aHeight", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString text; */
                    Method {
                        name: "GetText",
                        params: &[Param { name: "aText", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void getCharacterRect (in long offset, out long left, out long top, out long width, out long height); */
                    Method {
                        name: "GetCharacterRect",
                        params: &[Param { name: "offset", ty: "i32" }, Param { name: "left", ty: "*mut i32" }, Param { name: "top", ty: "*mut i32" }, Param { name: "width", ty: "*mut i32" }, Param { name: "height", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean succeeded; */
                    Method {
                        name: "GetSucceeded",
                        params: &[Param { name: "aSucceeded", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean notFound; */
                    Method {
                        name: "GetNotFound",
                        params: &[Param { name: "aNotFound", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean tentativeCaretOffsetNotFound; */
                    Method {
                        name: "GetTentativeCaretOffsetNotFound",
                        params: &[Param { name: "aTentativeCaretOffsetNotFound", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

