//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/uriloader/exthandler/nsISharingHandlerApp.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISharingHandlerApp",
            base: Some("nsIHandlerApp"),
            methods: Ok(&[
                    /* void share (in AString data, [optional] in AString title); */
                    Method {
                        name: "Share",
                        params: &[Param { name: "data", ty: "*const ::nsstring::nsAString" }, Param { name: "title", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

