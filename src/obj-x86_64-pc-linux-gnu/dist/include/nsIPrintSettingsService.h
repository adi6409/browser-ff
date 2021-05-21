/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/widget/nsIPrintSettingsService.idl
 */

#ifndef __gen_nsIPrintSettingsService_h__
#define __gen_nsIPrintSettingsService_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIPrintSettings; /* forward declaration */

class nsIWebBrowserPrint; /* forward declaration */

namespace mozilla {
namespace embedding {
  class PrintData;
}
}

/* starting interface:    nsIPrintSettingsService */
#define NS_IPRINTSETTINGSSERVICE_IID_STR "841387c8-72e6-484b-9296-bf6eea80d58a"

#define NS_IPRINTSETTINGSSERVICE_IID \
  {0x841387c8, 0x72e6, 0x484b, \
    { 0x92, 0x96, 0xbf, 0x6e, 0xea, 0x80, 0xd5, 0x8a }}

class NS_NO_VTABLE nsIPrintSettingsService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPRINTSETTINGSSERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPrintSettingsService;

  /* [noscript] readonly attribute nsIPrintSettings defaultPrintSettingsForPrinting; */
  NS_IMETHOD GetDefaultPrintSettingsForPrinting(nsIPrintSettings **aDefaultPrintSettingsForPrinting) = 0;

  /* readonly attribute nsIPrintSettings newPrintSettings; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetNewPrintSettings(nsIPrintSettings **aNewPrintSettings) = 0;

  /* readonly attribute AString lastUsedPrinterName; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetLastUsedPrinterName(nsAString& aLastUsedPrinterName) = 0;

  /* void initPrintSettingsFromPrinter (in AString aPrinterName, in nsIPrintSettings aPrintSettings); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD InitPrintSettingsFromPrinter(const nsAString& aPrinterName, nsIPrintSettings *aPrintSettings) = 0;

  /* void initPrintSettingsFromPrefs (in nsIPrintSettings aPrintSettings, in boolean aUsePrinterNamePrefix, in unsigned long aFlags); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD InitPrintSettingsFromPrefs(nsIPrintSettings *aPrintSettings, bool aUsePrinterNamePrefix, uint32_t aFlags) = 0;

  /* void savePrintSettingsToPrefs (in nsIPrintSettings aPrintSettings, in boolean aUsePrinterNamePrefix, in unsigned long aFlags); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SavePrintSettingsToPrefs(nsIPrintSettings *aPrintSettings, bool aUsePrinterNamePrefix, uint32_t aFlags) = 0;

  /* [noscript] void SerializeToPrintData (in nsIPrintSettings aPrintSettings, in PrintDataPtr data); */
  NS_IMETHOD SerializeToPrintData(nsIPrintSettings *aPrintSettings, mozilla::embedding::PrintData * data) = 0;

  /* [noscript] void DeserializeToPrintSettings (in PrintDataRef data, in nsIPrintSettings aPrintSettings); */
  NS_IMETHOD DeserializeToPrintSettings(const mozilla::embedding::PrintData & data, nsIPrintSettings *aPrintSettings) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPrintSettingsService, NS_IPRINTSETTINGSSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPRINTSETTINGSSERVICE \
  NS_IMETHOD GetDefaultPrintSettingsForPrinting(nsIPrintSettings **aDefaultPrintSettingsForPrinting) override; \
  NS_IMETHOD GetNewPrintSettings(nsIPrintSettings **aNewPrintSettings) override; \
  NS_IMETHOD GetLastUsedPrinterName(nsAString& aLastUsedPrinterName) override; \
  NS_IMETHOD InitPrintSettingsFromPrinter(const nsAString& aPrinterName, nsIPrintSettings *aPrintSettings) override; \
  NS_IMETHOD InitPrintSettingsFromPrefs(nsIPrintSettings *aPrintSettings, bool aUsePrinterNamePrefix, uint32_t aFlags) override; \
  NS_IMETHOD SavePrintSettingsToPrefs(nsIPrintSettings *aPrintSettings, bool aUsePrinterNamePrefix, uint32_t aFlags) override; \
  NS_IMETHOD SerializeToPrintData(nsIPrintSettings *aPrintSettings, mozilla::embedding::PrintData * data) override; \
  NS_IMETHOD DeserializeToPrintSettings(const mozilla::embedding::PrintData & data, nsIPrintSettings *aPrintSettings) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPRINTSETTINGSSERVICE \
  nsresult GetDefaultPrintSettingsForPrinting(nsIPrintSettings **aDefaultPrintSettingsForPrinting); \
  nsresult GetNewPrintSettings(nsIPrintSettings **aNewPrintSettings); \
  nsresult GetLastUsedPrinterName(nsAString& aLastUsedPrinterName); \
  nsresult InitPrintSettingsFromPrinter(const nsAString& aPrinterName, nsIPrintSettings *aPrintSettings); \
  nsresult InitPrintSettingsFromPrefs(nsIPrintSettings *aPrintSettings, bool aUsePrinterNamePrefix, uint32_t aFlags); \
  nsresult SavePrintSettingsToPrefs(nsIPrintSettings *aPrintSettings, bool aUsePrinterNamePrefix, uint32_t aFlags); \
  nsresult SerializeToPrintData(nsIPrintSettings *aPrintSettings, mozilla::embedding::PrintData * data); \
  nsresult DeserializeToPrintSettings(const mozilla::embedding::PrintData & data, nsIPrintSettings *aPrintSettings); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPRINTSETTINGSSERVICE(_to) \
  NS_IMETHOD GetDefaultPrintSettingsForPrinting(nsIPrintSettings **aDefaultPrintSettingsForPrinting) override { return _to GetDefaultPrintSettingsForPrinting(aDefaultPrintSettingsForPrinting); } \
  NS_IMETHOD GetNewPrintSettings(nsIPrintSettings **aNewPrintSettings) override { return _to GetNewPrintSettings(aNewPrintSettings); } \
  NS_IMETHOD GetLastUsedPrinterName(nsAString& aLastUsedPrinterName) override { return _to GetLastUsedPrinterName(aLastUsedPrinterName); } \
  NS_IMETHOD InitPrintSettingsFromPrinter(const nsAString& aPrinterName, nsIPrintSettings *aPrintSettings) override { return _to InitPrintSettingsFromPrinter(aPrinterName, aPrintSettings); } \
  NS_IMETHOD InitPrintSettingsFromPrefs(nsIPrintSettings *aPrintSettings, bool aUsePrinterNamePrefix, uint32_t aFlags) override { return _to InitPrintSettingsFromPrefs(aPrintSettings, aUsePrinterNamePrefix, aFlags); } \
  NS_IMETHOD SavePrintSettingsToPrefs(nsIPrintSettings *aPrintSettings, bool aUsePrinterNamePrefix, uint32_t aFlags) override { return _to SavePrintSettingsToPrefs(aPrintSettings, aUsePrinterNamePrefix, aFlags); } \
  NS_IMETHOD SerializeToPrintData(nsIPrintSettings *aPrintSettings, mozilla::embedding::PrintData * data) override { return _to SerializeToPrintData(aPrintSettings, data); } \
  NS_IMETHOD DeserializeToPrintSettings(const mozilla::embedding::PrintData & data, nsIPrintSettings *aPrintSettings) override { return _to DeserializeToPrintSettings(data, aPrintSettings); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPRINTSETTINGSSERVICE(_to) \
  NS_IMETHOD GetDefaultPrintSettingsForPrinting(nsIPrintSettings **aDefaultPrintSettingsForPrinting) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDefaultPrintSettingsForPrinting(aDefaultPrintSettingsForPrinting); } \
  NS_IMETHOD GetNewPrintSettings(nsIPrintSettings **aNewPrintSettings) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNewPrintSettings(aNewPrintSettings); } \
  NS_IMETHOD GetLastUsedPrinterName(nsAString& aLastUsedPrinterName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLastUsedPrinterName(aLastUsedPrinterName); } \
  NS_IMETHOD InitPrintSettingsFromPrinter(const nsAString& aPrinterName, nsIPrintSettings *aPrintSettings) override { return !_to ? NS_ERROR_NULL_POINTER : _to->InitPrintSettingsFromPrinter(aPrinterName, aPrintSettings); } \
  NS_IMETHOD InitPrintSettingsFromPrefs(nsIPrintSettings *aPrintSettings, bool aUsePrinterNamePrefix, uint32_t aFlags) override { return !_to ? NS_ERROR_NULL_POINTER : _to->InitPrintSettingsFromPrefs(aPrintSettings, aUsePrinterNamePrefix, aFlags); } \
  NS_IMETHOD SavePrintSettingsToPrefs(nsIPrintSettings *aPrintSettings, bool aUsePrinterNamePrefix, uint32_t aFlags) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SavePrintSettingsToPrefs(aPrintSettings, aUsePrinterNamePrefix, aFlags); } \
  NS_IMETHOD SerializeToPrintData(nsIPrintSettings *aPrintSettings, mozilla::embedding::PrintData * data) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SerializeToPrintData(aPrintSettings, data); } \
  NS_IMETHOD DeserializeToPrintSettings(const mozilla::embedding::PrintData & data, nsIPrintSettings *aPrintSettings) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DeserializeToPrintSettings(data, aPrintSettings); } 

// {841387C8-72E6-484b-9296-BF6EEA80D58A}
#define NS_PRINTSETTINGSSERVICE_IID \
 {0x841387c8, 0x72e6, 0x484b, { 0x92, 0x96, 0xbf, 0x6e, 0xea, 0x80, 0xd5, 0x8a}}

#endif /* __gen_nsIPrintSettingsService_h__ */
