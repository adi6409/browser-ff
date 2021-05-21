//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/xul/nsIDOMXULButtonElement.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMXULButtonElement",
            base: Some("nsIDOMXULControlElement"),
            methods: Ok(&[
                    /* attribute AString type; */
                    Method {
                        name: "GetType",
                        params: &[Param { name: "aType", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetType",
                        params: &[Param { name: "aType", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute boolean open; */
                    Method {
                        name: "GetOpen",
                        params: &[Param { name: "aOpen", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetOpen",
                        params: &[Param { name: "aOpen", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute boolean checked; */
                    Method {
                        name: "GetChecked",
                        params: &[Param { name: "aChecked", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetChecked",
                        params: &[Param { name: "aChecked", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute AString group; */
                    Method {
                        name: "GetGroup",
                        params: &[Param { name: "aGroup", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetGroup",
                        params: &[Param { name: "aGroup", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

