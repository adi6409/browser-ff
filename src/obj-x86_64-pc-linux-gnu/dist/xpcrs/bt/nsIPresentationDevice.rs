//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/presentation/interfaces/nsIPresentationDevice.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPresentationDevice",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute AUTF8String id; */
                    Method {
                        name: "GetId",
                        params: &[Param { name: "aId", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AUTF8String name; */
                    Method {
                        name: "GetName",
                        params: &[Param { name: "aName", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AUTF8String type; */
                    Method {
                        name: "GetType",
                        params: &[Param { name: "aType", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIPresentationControlChannel establishControlChannel (); */
                    Method {
                        name: "EstablishControlChannel",
                        params: &[Param { name: "_retval", ty: "*mut*const nsIPresentationControlChannel" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void disconnect (); */
                    Method {
                        name: "Disconnect",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean isRequestedUrlSupported (in AString requestedUrl); */
                    Method {
                        name: "IsRequestedUrlSupported",
                        params: &[Param { name: "requestedUrl", ty: "*const ::nsstring::nsAString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

