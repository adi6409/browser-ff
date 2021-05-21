/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/layout/base/nsIStyleSheetService.idl
 */

#ifndef __gen_nsIStyleSheetService_h__
#define __gen_nsIStyleSheetService_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/Value.h"

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIPreloadedStyleSheet; /* forward declaration */

class nsIURI; /* forward declaration */


/* starting interface:    nsIStyleSheetService */
#define NS_ISTYLESHEETSERVICE_IID_STR "4de68896-e8eb-41de-8237-a797b570ac4a"

#define NS_ISTYLESHEETSERVICE_IID \
  {0x4de68896, 0xe8eb, 0x41de, \
    { 0x82, 0x37, 0xa7, 0x97, 0xb5, 0x70, 0xac, 0x4a }}

class NS_NO_VTABLE nsIStyleSheetService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISTYLESHEETSERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIStyleSheetService;

  enum {
    AGENT_SHEET = 0U,
    USER_SHEET = 1U,
    AUTHOR_SHEET = 2U
  };

  /* void loadAndRegisterSheet (in nsIURI sheetURI, in unsigned long type); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD LoadAndRegisterSheet(nsIURI *sheetURI, uint32_t type) = 0;

  /* boolean sheetRegistered (in nsIURI sheetURI, in unsigned long type); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SheetRegistered(nsIURI *sheetURI, uint32_t type, bool *_retval) = 0;

  /* nsIPreloadedStyleSheet preloadSheet (in nsIURI sheetURI, in unsigned long type); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD PreloadSheet(nsIURI *sheetURI, uint32_t type, nsIPreloadedStyleSheet **_retval) = 0;

  /* [implicit_jscontext] jsval preloadSheetAsync (in nsIURI sheetURI, in unsigned long type); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD PreloadSheetAsync(nsIURI *sheetURI, uint32_t type, JSContext* cx, JS::MutableHandleValue _retval) = 0;

  /* void unregisterSheet (in nsIURI sheetURI, in unsigned long type); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD UnregisterSheet(nsIURI *sheetURI, uint32_t type) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIStyleSheetService, NS_ISTYLESHEETSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISTYLESHEETSERVICE \
  NS_IMETHOD LoadAndRegisterSheet(nsIURI *sheetURI, uint32_t type) override; \
  NS_IMETHOD SheetRegistered(nsIURI *sheetURI, uint32_t type, bool *_retval) override; \
  NS_IMETHOD PreloadSheet(nsIURI *sheetURI, uint32_t type, nsIPreloadedStyleSheet **_retval) override; \
  NS_IMETHOD PreloadSheetAsync(nsIURI *sheetURI, uint32_t type, JSContext* cx, JS::MutableHandleValue _retval) override; \
  NS_IMETHOD UnregisterSheet(nsIURI *sheetURI, uint32_t type) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISTYLESHEETSERVICE \
  nsresult LoadAndRegisterSheet(nsIURI *sheetURI, uint32_t type); \
  nsresult SheetRegistered(nsIURI *sheetURI, uint32_t type, bool *_retval); \
  nsresult PreloadSheet(nsIURI *sheetURI, uint32_t type, nsIPreloadedStyleSheet **_retval); \
  nsresult PreloadSheetAsync(nsIURI *sheetURI, uint32_t type, JSContext* cx, JS::MutableHandleValue _retval); \
  nsresult UnregisterSheet(nsIURI *sheetURI, uint32_t type); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISTYLESHEETSERVICE(_to) \
  NS_IMETHOD LoadAndRegisterSheet(nsIURI *sheetURI, uint32_t type) override { return _to LoadAndRegisterSheet(sheetURI, type); } \
  NS_IMETHOD SheetRegistered(nsIURI *sheetURI, uint32_t type, bool *_retval) override { return _to SheetRegistered(sheetURI, type, _retval); } \
  NS_IMETHOD PreloadSheet(nsIURI *sheetURI, uint32_t type, nsIPreloadedStyleSheet **_retval) override { return _to PreloadSheet(sheetURI, type, _retval); } \
  NS_IMETHOD PreloadSheetAsync(nsIURI *sheetURI, uint32_t type, JSContext* cx, JS::MutableHandleValue _retval) override { return _to PreloadSheetAsync(sheetURI, type, cx, _retval); } \
  NS_IMETHOD UnregisterSheet(nsIURI *sheetURI, uint32_t type) override { return _to UnregisterSheet(sheetURI, type); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISTYLESHEETSERVICE(_to) \
  NS_IMETHOD LoadAndRegisterSheet(nsIURI *sheetURI, uint32_t type) override { return !_to ? NS_ERROR_NULL_POINTER : _to->LoadAndRegisterSheet(sheetURI, type); } \
  NS_IMETHOD SheetRegistered(nsIURI *sheetURI, uint32_t type, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SheetRegistered(sheetURI, type, _retval); } \
  NS_IMETHOD PreloadSheet(nsIURI *sheetURI, uint32_t type, nsIPreloadedStyleSheet **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->PreloadSheet(sheetURI, type, _retval); } \
  NS_IMETHOD PreloadSheetAsync(nsIURI *sheetURI, uint32_t type, JSContext* cx, JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->PreloadSheetAsync(sheetURI, type, cx, _retval); } \
  NS_IMETHOD UnregisterSheet(nsIURI *sheetURI, uint32_t type) override { return !_to ? NS_ERROR_NULL_POINTER : _to->UnregisterSheet(sheetURI, type); } 


#endif /* __gen_nsIStyleSheetService_h__ */
