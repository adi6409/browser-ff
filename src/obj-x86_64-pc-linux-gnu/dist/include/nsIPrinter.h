/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/widget/nsIPrinter.idl
 */

#ifndef __gen_nsIPrinter_h__
#define __gen_nsIPrinter_h__


#ifndef __gen_nsIPaper_h__
#include "nsIPaper.h"
#endif

#ifndef __gen_nsIPrintSettings_h__
#include "nsIPrintSettings.h"
#endif

#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsTArray_h__
#include "nsTArray.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIPrinterInfo */
#define NS_IPRINTERINFO_IID_STR "855ae9dd-62a4-64aa-9c60-b1078ff028f1"

#define NS_IPRINTERINFO_IID \
  {0x855ae9dd, 0x62a4, 0x64aa, \
    { 0x9c, 0x60, 0xb1, 0x07, 0x8f, 0xf0, 0x28, 0xf1 }}

class NS_NO_VTABLE nsIPrinterInfo : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPRINTERINFO_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPrinterInfo;

  /* readonly attribute Array<nsIPaper> paperList; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPaperList(nsTArray<RefPtr<nsIPaper>>& aPaperList) = 0;

  /* readonly attribute nsIPrintSettings defaultSettings; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDefaultSettings(nsIPrintSettings **aDefaultSettings) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPrinterInfo, NS_IPRINTERINFO_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPRINTERINFO \
  NS_IMETHOD GetPaperList(nsTArray<RefPtr<nsIPaper>>& aPaperList) override; \
  NS_IMETHOD GetDefaultSettings(nsIPrintSettings **aDefaultSettings) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPRINTERINFO \
  nsresult GetPaperList(nsTArray<RefPtr<nsIPaper>>& aPaperList); \
  nsresult GetDefaultSettings(nsIPrintSettings **aDefaultSettings); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPRINTERINFO(_to) \
  NS_IMETHOD GetPaperList(nsTArray<RefPtr<nsIPaper>>& aPaperList) override { return _to GetPaperList(aPaperList); } \
  NS_IMETHOD GetDefaultSettings(nsIPrintSettings **aDefaultSettings) override { return _to GetDefaultSettings(aDefaultSettings); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPRINTERINFO(_to) \
  NS_IMETHOD GetPaperList(nsTArray<RefPtr<nsIPaper>>& aPaperList) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPaperList(aPaperList); } \
  NS_IMETHOD GetDefaultSettings(nsIPrintSettings **aDefaultSettings) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDefaultSettings(aDefaultSettings); } 


/* starting interface:    nsIPrinter */
#define NS_IPRINTER_IID_STR "d2dde9bb-df86-469c-bfcc-fd95a44b1db8"

#define NS_IPRINTER_IID \
  {0xd2dde9bb, 0xdf86, 0x469c, \
    { 0xbf, 0xcc, 0xfd, 0x95, 0xa4, 0x4b, 0x1d, 0xb8 }}

class NS_NO_VTABLE nsIPrinter : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPRINTER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPrinter;

  /* readonly attribute AString name; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetName(nsAString& aName) = 0;

  /* readonly attribute AString systemName; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSystemName(nsAString& aSystemName) = 0;

  /* [implicit_jscontext] readonly attribute Promise printerInfo; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPrinterInfo(JSContext* cx, ::mozilla::dom::Promise * * aPrinterInfo) = 0;

  /* [implicit_jscontext] readonly attribute Promise supportsDuplex; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSupportsDuplex(JSContext* cx, ::mozilla::dom::Promise * * aSupportsDuplex) = 0;

  /* [implicit_jscontext] readonly attribute Promise supportsColor; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSupportsColor(JSContext* cx, ::mozilla::dom::Promise * * aSupportsColor) = 0;

  /* [implicit_jscontext] readonly attribute Promise supportsMonochrome; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSupportsMonochrome(JSContext* cx, ::mozilla::dom::Promise * * aSupportsMonochrome) = 0;

  /* [implicit_jscontext] readonly attribute Promise supportsCollation; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSupportsCollation(JSContext* cx, ::mozilla::dom::Promise * * aSupportsCollation) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPrinter, NS_IPRINTER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPRINTER \
  NS_IMETHOD GetName(nsAString& aName) override; \
  NS_IMETHOD GetSystemName(nsAString& aSystemName) override; \
  NS_IMETHOD GetPrinterInfo(JSContext* cx, ::mozilla::dom::Promise * * aPrinterInfo) override; \
  NS_IMETHOD GetSupportsDuplex(JSContext* cx, ::mozilla::dom::Promise * * aSupportsDuplex) override; \
  NS_IMETHOD GetSupportsColor(JSContext* cx, ::mozilla::dom::Promise * * aSupportsColor) override; \
  NS_IMETHOD GetSupportsMonochrome(JSContext* cx, ::mozilla::dom::Promise * * aSupportsMonochrome) override; \
  NS_IMETHOD GetSupportsCollation(JSContext* cx, ::mozilla::dom::Promise * * aSupportsCollation) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPRINTER \
  nsresult GetName(nsAString& aName); \
  nsresult GetSystemName(nsAString& aSystemName); \
  nsresult GetPrinterInfo(JSContext* cx, ::mozilla::dom::Promise * * aPrinterInfo); \
  nsresult GetSupportsDuplex(JSContext* cx, ::mozilla::dom::Promise * * aSupportsDuplex); \
  nsresult GetSupportsColor(JSContext* cx, ::mozilla::dom::Promise * * aSupportsColor); \
  nsresult GetSupportsMonochrome(JSContext* cx, ::mozilla::dom::Promise * * aSupportsMonochrome); \
  nsresult GetSupportsCollation(JSContext* cx, ::mozilla::dom::Promise * * aSupportsCollation); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPRINTER(_to) \
  NS_IMETHOD GetName(nsAString& aName) override { return _to GetName(aName); } \
  NS_IMETHOD GetSystemName(nsAString& aSystemName) override { return _to GetSystemName(aSystemName); } \
  NS_IMETHOD GetPrinterInfo(JSContext* cx, ::mozilla::dom::Promise * * aPrinterInfo) override { return _to GetPrinterInfo(cx, aPrinterInfo); } \
  NS_IMETHOD GetSupportsDuplex(JSContext* cx, ::mozilla::dom::Promise * * aSupportsDuplex) override { return _to GetSupportsDuplex(cx, aSupportsDuplex); } \
  NS_IMETHOD GetSupportsColor(JSContext* cx, ::mozilla::dom::Promise * * aSupportsColor) override { return _to GetSupportsColor(cx, aSupportsColor); } \
  NS_IMETHOD GetSupportsMonochrome(JSContext* cx, ::mozilla::dom::Promise * * aSupportsMonochrome) override { return _to GetSupportsMonochrome(cx, aSupportsMonochrome); } \
  NS_IMETHOD GetSupportsCollation(JSContext* cx, ::mozilla::dom::Promise * * aSupportsCollation) override { return _to GetSupportsCollation(cx, aSupportsCollation); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPRINTER(_to) \
  NS_IMETHOD GetName(nsAString& aName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetName(aName); } \
  NS_IMETHOD GetSystemName(nsAString& aSystemName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSystemName(aSystemName); } \
  NS_IMETHOD GetPrinterInfo(JSContext* cx, ::mozilla::dom::Promise * * aPrinterInfo) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPrinterInfo(cx, aPrinterInfo); } \
  NS_IMETHOD GetSupportsDuplex(JSContext* cx, ::mozilla::dom::Promise * * aSupportsDuplex) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSupportsDuplex(cx, aSupportsDuplex); } \
  NS_IMETHOD GetSupportsColor(JSContext* cx, ::mozilla::dom::Promise * * aSupportsColor) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSupportsColor(cx, aSupportsColor); } \
  NS_IMETHOD GetSupportsMonochrome(JSContext* cx, ::mozilla::dom::Promise * * aSupportsMonochrome) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSupportsMonochrome(cx, aSupportsMonochrome); } \
  NS_IMETHOD GetSupportsCollation(JSContext* cx, ::mozilla::dom::Promise * * aSupportsCollation) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSupportsCollation(cx, aSupportsCollation); } 


#endif /* __gen_nsIPrinter_h__ */
