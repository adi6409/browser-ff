/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/widget/nsIPrinterList.idl
 */

#ifndef __gen_nsIPrinterList_h__
#define __gen_nsIPrinterList_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsIPrinter_h__
#include "nsIPrinter.h"
#endif

#ifndef __gen_nsIPrintSettings_h__
#include "nsIPrintSettings.h"
#endif

#include "js/Value.h"

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIPrinterList */
#define NS_IPRINTERLIST_IID_STR "5e738fff-404c-4c94-9189-e8f2cce93e94"

#define NS_IPRINTERLIST_IID \
  {0x5e738fff, 0x404c, 0x4c94, \
    { 0x91, 0x89, 0xe8, 0xf2, 0xcc, 0xe9, 0x3e, 0x94 }}

class NS_NO_VTABLE nsIPrinterList : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPRINTERLIST_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPrinterList;

  /* void initPrintSettingsFromPrinter (in AString aPrinterName, in nsIPrintSettings aPrintSettings); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD InitPrintSettingsFromPrinter(const nsAString& aPrinterName, nsIPrintSettings *aPrintSettings) = 0;

  /* readonly attribute AString systemDefaultPrinterName; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSystemDefaultPrinterName(nsAString& aSystemDefaultPrinterName) = 0;

  /* [implicit_jscontext] Promise getPrinterByName (in AString aPrinterName); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPrinterByName(const nsAString& aPrinterName, JSContext* cx, ::mozilla::dom::Promise * * _retval) = 0;

  /* [implicit_jscontext] Promise getPrinterBySystemName (in AString aPrinterName); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPrinterBySystemName(const nsAString& aPrinterName, JSContext* cx, ::mozilla::dom::Promise * * _retval) = 0;

  /* [implicit_jscontext] Promise getNamedOrDefaultPrinter (in AString aPrinterName); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetNamedOrDefaultPrinter(const nsAString& aPrinterName, JSContext* cx, ::mozilla::dom::Promise * * _retval) = 0;

  /* [implicit_jscontext] readonly attribute Promise printers; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPrinters(JSContext* cx, ::mozilla::dom::Promise * * aPrinters) = 0;

  /* [implicit_jscontext] readonly attribute Promise fallbackPaperList; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetFallbackPaperList(JSContext* cx, ::mozilla::dom::Promise * * aFallbackPaperList) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPrinterList, NS_IPRINTERLIST_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPRINTERLIST \
  NS_IMETHOD InitPrintSettingsFromPrinter(const nsAString& aPrinterName, nsIPrintSettings *aPrintSettings) override; \
  NS_IMETHOD GetSystemDefaultPrinterName(nsAString& aSystemDefaultPrinterName) override; \
  NS_IMETHOD GetPrinterByName(const nsAString& aPrinterName, JSContext* cx, ::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD GetPrinterBySystemName(const nsAString& aPrinterName, JSContext* cx, ::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD GetNamedOrDefaultPrinter(const nsAString& aPrinterName, JSContext* cx, ::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD GetPrinters(JSContext* cx, ::mozilla::dom::Promise * * aPrinters) override; \
  NS_IMETHOD GetFallbackPaperList(JSContext* cx, ::mozilla::dom::Promise * * aFallbackPaperList) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPRINTERLIST \
  nsresult InitPrintSettingsFromPrinter(const nsAString& aPrinterName, nsIPrintSettings *aPrintSettings); \
  nsresult GetSystemDefaultPrinterName(nsAString& aSystemDefaultPrinterName); \
  nsresult GetPrinterByName(const nsAString& aPrinterName, JSContext* cx, ::mozilla::dom::Promise * * _retval); \
  nsresult GetPrinterBySystemName(const nsAString& aPrinterName, JSContext* cx, ::mozilla::dom::Promise * * _retval); \
  nsresult GetNamedOrDefaultPrinter(const nsAString& aPrinterName, JSContext* cx, ::mozilla::dom::Promise * * _retval); \
  nsresult GetPrinters(JSContext* cx, ::mozilla::dom::Promise * * aPrinters); \
  nsresult GetFallbackPaperList(JSContext* cx, ::mozilla::dom::Promise * * aFallbackPaperList); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPRINTERLIST(_to) \
  NS_IMETHOD InitPrintSettingsFromPrinter(const nsAString& aPrinterName, nsIPrintSettings *aPrintSettings) override { return _to InitPrintSettingsFromPrinter(aPrinterName, aPrintSettings); } \
  NS_IMETHOD GetSystemDefaultPrinterName(nsAString& aSystemDefaultPrinterName) override { return _to GetSystemDefaultPrinterName(aSystemDefaultPrinterName); } \
  NS_IMETHOD GetPrinterByName(const nsAString& aPrinterName, JSContext* cx, ::mozilla::dom::Promise * * _retval) override { return _to GetPrinterByName(aPrinterName, cx, _retval); } \
  NS_IMETHOD GetPrinterBySystemName(const nsAString& aPrinterName, JSContext* cx, ::mozilla::dom::Promise * * _retval) override { return _to GetPrinterBySystemName(aPrinterName, cx, _retval); } \
  NS_IMETHOD GetNamedOrDefaultPrinter(const nsAString& aPrinterName, JSContext* cx, ::mozilla::dom::Promise * * _retval) override { return _to GetNamedOrDefaultPrinter(aPrinterName, cx, _retval); } \
  NS_IMETHOD GetPrinters(JSContext* cx, ::mozilla::dom::Promise * * aPrinters) override { return _to GetPrinters(cx, aPrinters); } \
  NS_IMETHOD GetFallbackPaperList(JSContext* cx, ::mozilla::dom::Promise * * aFallbackPaperList) override { return _to GetFallbackPaperList(cx, aFallbackPaperList); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPRINTERLIST(_to) \
  NS_IMETHOD InitPrintSettingsFromPrinter(const nsAString& aPrinterName, nsIPrintSettings *aPrintSettings) override { return !_to ? NS_ERROR_NULL_POINTER : _to->InitPrintSettingsFromPrinter(aPrinterName, aPrintSettings); } \
  NS_IMETHOD GetSystemDefaultPrinterName(nsAString& aSystemDefaultPrinterName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSystemDefaultPrinterName(aSystemDefaultPrinterName); } \
  NS_IMETHOD GetPrinterByName(const nsAString& aPrinterName, JSContext* cx, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPrinterByName(aPrinterName, cx, _retval); } \
  NS_IMETHOD GetPrinterBySystemName(const nsAString& aPrinterName, JSContext* cx, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPrinterBySystemName(aPrinterName, cx, _retval); } \
  NS_IMETHOD GetNamedOrDefaultPrinter(const nsAString& aPrinterName, JSContext* cx, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNamedOrDefaultPrinter(aPrinterName, cx, _retval); } \
  NS_IMETHOD GetPrinters(JSContext* cx, ::mozilla::dom::Promise * * aPrinters) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPrinters(cx, aPrinters); } \
  NS_IMETHOD GetFallbackPaperList(JSContext* cx, ::mozilla::dom::Promise * * aFallbackPaperList) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFallbackPaperList(cx, aFallbackPaperList); } 


#endif /* __gen_nsIPrinterList_h__ */
