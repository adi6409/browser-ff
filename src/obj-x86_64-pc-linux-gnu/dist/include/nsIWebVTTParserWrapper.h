/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/media/webvtt/nsIWebVTTParserWrapper.idl
 */

#ifndef __gen_nsIWebVTTParserWrapper_h__
#define __gen_nsIWebVTTParserWrapper_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIWebVTTListener; /* forward declaration */

class mozIDOMWindow; /* forward declaration */

class nsIVariant; /* forward declaration */

namespace mozilla {
namespace dom {
class DocumentFragment; /* webidl DocumentFragment */
} // namespace dom
} // namespace mozilla


/* starting interface:    nsIWebVTTParserWrapper */
#define NS_IWEBVTTPARSERWRAPPER_IID_STR "8dfe016e-1701-4618-9f5e-9a6154e853f0"

#define NS_IWEBVTTPARSERWRAPPER_IID \
  {0x8dfe016e, 0x1701, 0x4618, \
    { 0x9f, 0x5e, 0x9a, 0x61, 0x54, 0xe8, 0x53, 0xf0 }}

class NS_NO_VTABLE nsIWebVTTParserWrapper : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IWEBVTTPARSERWRAPPER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIWebVTTParserWrapper;

  /* void loadParser (in mozIDOMWindow window); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD LoadParser(mozIDOMWindow *window) = 0;

  /* void parse (in ACString data); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Parse(const nsACString& data) = 0;

  /* void flush (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Flush(void) = 0;

  /* void watch (in nsIWebVTTListener callback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Watch(nsIWebVTTListener *callback) = 0;

  /* void cancel (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Cancel(void) = 0;

  /* DocumentFragment convertCueToDOMTree (in mozIDOMWindow window, in nsISupports cue); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ConvertCueToDOMTree(mozIDOMWindow *window, nsISupports *cue, mozilla::dom::DocumentFragment **_retval) = 0;

  /* void processCues (in mozIDOMWindow window, in nsIVariant cues, in nsISupports overlay, in nsISupports controls); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ProcessCues(mozIDOMWindow *window, nsIVariant *cues, nsISupports *overlay, nsISupports *controls) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIWebVTTParserWrapper, NS_IWEBVTTPARSERWRAPPER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIWEBVTTPARSERWRAPPER \
  NS_IMETHOD LoadParser(mozIDOMWindow *window) override; \
  NS_IMETHOD Parse(const nsACString& data) override; \
  NS_IMETHOD Flush(void) override; \
  NS_IMETHOD Watch(nsIWebVTTListener *callback) override; \
  NS_IMETHOD Cancel(void) override; \
  NS_IMETHOD ConvertCueToDOMTree(mozIDOMWindow *window, nsISupports *cue, mozilla::dom::DocumentFragment **_retval) override; \
  NS_IMETHOD ProcessCues(mozIDOMWindow *window, nsIVariant *cues, nsISupports *overlay, nsISupports *controls) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIWEBVTTPARSERWRAPPER \
  nsresult LoadParser(mozIDOMWindow *window); \
  nsresult Parse(const nsACString& data); \
  nsresult Flush(void); \
  nsresult Watch(nsIWebVTTListener *callback); \
  nsresult Cancel(void); \
  nsresult ConvertCueToDOMTree(mozIDOMWindow *window, nsISupports *cue, mozilla::dom::DocumentFragment **_retval); \
  nsresult ProcessCues(mozIDOMWindow *window, nsIVariant *cues, nsISupports *overlay, nsISupports *controls); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIWEBVTTPARSERWRAPPER(_to) \
  NS_IMETHOD LoadParser(mozIDOMWindow *window) override { return _to LoadParser(window); } \
  NS_IMETHOD Parse(const nsACString& data) override { return _to Parse(data); } \
  NS_IMETHOD Flush(void) override { return _to Flush(); } \
  NS_IMETHOD Watch(nsIWebVTTListener *callback) override { return _to Watch(callback); } \
  NS_IMETHOD Cancel(void) override { return _to Cancel(); } \
  NS_IMETHOD ConvertCueToDOMTree(mozIDOMWindow *window, nsISupports *cue, mozilla::dom::DocumentFragment **_retval) override { return _to ConvertCueToDOMTree(window, cue, _retval); } \
  NS_IMETHOD ProcessCues(mozIDOMWindow *window, nsIVariant *cues, nsISupports *overlay, nsISupports *controls) override { return _to ProcessCues(window, cues, overlay, controls); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIWEBVTTPARSERWRAPPER(_to) \
  NS_IMETHOD LoadParser(mozIDOMWindow *window) override { return !_to ? NS_ERROR_NULL_POINTER : _to->LoadParser(window); } \
  NS_IMETHOD Parse(const nsACString& data) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Parse(data); } \
  NS_IMETHOD Flush(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Flush(); } \
  NS_IMETHOD Watch(nsIWebVTTListener *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Watch(callback); } \
  NS_IMETHOD Cancel(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Cancel(); } \
  NS_IMETHOD ConvertCueToDOMTree(mozIDOMWindow *window, nsISupports *cue, mozilla::dom::DocumentFragment **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ConvertCueToDOMTree(window, cue, _retval); } \
  NS_IMETHOD ProcessCues(mozIDOMWindow *window, nsIVariant *cues, nsISupports *overlay, nsISupports *controls) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ProcessCues(window, cues, overlay, controls); } 

#define NS_WEBVTTPARSERWRAPPER_CONTRACTID "@mozilla.org/webvttParserWrapper;1"

#endif /* __gen_nsIWebVTTParserWrapper_h__ */
