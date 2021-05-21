/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/extensions/mozIExtensionProcessScript.idl
 */

#ifndef __gen_mozIExtensionProcessScript_h__
#define __gen_mozIExtensionProcessScript_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class mozIDOMWindow; /* forward declaration */

namespace mozilla {
namespace dom {
class Document; /* webidl Document */
} // namespace dom
} // namespace mozilla

namespace mozilla {
namespace extensions {
class WebExtensionContentScript; /* webidl WebExtensionContentScript */
} // namespace extensions
} // namespace mozilla


/* starting interface:    mozIExtensionProcessScript */
#define MOZIEXTENSIONPROCESSSCRIPT_IID_STR "6b09dc51-6caa-4ca7-9d6d-30c87258a630"

#define MOZIEXTENSIONPROCESSSCRIPT_IID \
  {0x6b09dc51, 0x6caa, 0x4ca7, \
    { 0x9d, 0x6d, 0x30, 0xc8, 0x72, 0x58, 0xa6, 0x30 }}

class NS_NO_VTABLE mozIExtensionProcessScript : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(MOZIEXTENSIONPROCESSSCRIPT_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = mozIExtensionProcessScript;

  /* void preloadContentScript (in nsISupports contentScript); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD PreloadContentScript(nsISupports *contentScript) = 0;

  /* Promise loadContentScript (in WebExtensionContentScript contentScript, in mozIDOMWindow window); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD LoadContentScript(mozilla::extensions::WebExtensionContentScript *contentScript, mozIDOMWindow *window, ::mozilla::dom::Promise * * _retval) = 0;

  /* void initExtensionDocument (in nsISupports extension, in Document doc, in bool privileged); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD InitExtensionDocument(nsISupports *extension, mozilla::dom::Document *doc, bool privileged) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(mozIExtensionProcessScript, MOZIEXTENSIONPROCESSSCRIPT_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_MOZIEXTENSIONPROCESSSCRIPT \
  NS_IMETHOD PreloadContentScript(nsISupports *contentScript) override; \
  NS_IMETHOD LoadContentScript(mozilla::extensions::WebExtensionContentScript *contentScript, mozIDOMWindow *window, ::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD InitExtensionDocument(nsISupports *extension, mozilla::dom::Document *doc, bool privileged) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_MOZIEXTENSIONPROCESSSCRIPT \
  nsresult PreloadContentScript(nsISupports *contentScript); \
  nsresult LoadContentScript(mozilla::extensions::WebExtensionContentScript *contentScript, mozIDOMWindow *window, ::mozilla::dom::Promise * * _retval); \
  nsresult InitExtensionDocument(nsISupports *extension, mozilla::dom::Document *doc, bool privileged); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_MOZIEXTENSIONPROCESSSCRIPT(_to) \
  NS_IMETHOD PreloadContentScript(nsISupports *contentScript) override { return _to PreloadContentScript(contentScript); } \
  NS_IMETHOD LoadContentScript(mozilla::extensions::WebExtensionContentScript *contentScript, mozIDOMWindow *window, ::mozilla::dom::Promise * * _retval) override { return _to LoadContentScript(contentScript, window, _retval); } \
  NS_IMETHOD InitExtensionDocument(nsISupports *extension, mozilla::dom::Document *doc, bool privileged) override { return _to InitExtensionDocument(extension, doc, privileged); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_MOZIEXTENSIONPROCESSSCRIPT(_to) \
  NS_IMETHOD PreloadContentScript(nsISupports *contentScript) override { return !_to ? NS_ERROR_NULL_POINTER : _to->PreloadContentScript(contentScript); } \
  NS_IMETHOD LoadContentScript(mozilla::extensions::WebExtensionContentScript *contentScript, mozIDOMWindow *window, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->LoadContentScript(contentScript, window, _retval); } \
  NS_IMETHOD InitExtensionDocument(nsISupports *extension, mozilla::dom::Document *doc, bool privileged) override { return !_to ? NS_ERROR_NULL_POINTER : _to->InitExtensionDocument(extension, doc, privileged); } 


#endif /* __gen_mozIExtensionProcessScript_h__ */
