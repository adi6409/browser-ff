/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/browser/nsIWebBrowserPrint.idl
 */

#ifndef __gen_nsIWebBrowserPrint_h__
#define __gen_nsIWebBrowserPrint_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
#include <functional>
namespace mozilla {
namespace dom {
class PrintPreviewResultInfo;
} // namespace dom
} // namespace mozilla
class mozIDOMWindowProxy; /* forward declaration */

class nsIPrintSettings; /* forward declaration */

class nsIWebProgressListener; /* forward declaration */


/* starting interface:    nsIWebBrowserPrint */
#define NS_IWEBBROWSERPRINT_IID_STR "c9a934ed-fff1-4971-bfba-6c25ad70e1e6"

#define NS_IWEBBROWSERPRINT_IID \
  {0xc9a934ed, 0xfff1, 0x4971, \
    { 0xbf, 0xba, 0x6c, 0x25, 0xad, 0x70, 0xe1, 0xe6 }}

class NS_NO_VTABLE nsIWebBrowserPrint : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IWEBBROWSERPRINT_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIWebBrowserPrint;

  enum {
    PRINTPREVIEW_GOTO_PAGENUM = 0,
    PRINTPREVIEW_PREV_PAGE = 1,
    PRINTPREVIEW_NEXT_PAGE = 2,
    PRINTPREVIEW_HOME = 3,
    PRINTPREVIEW_END = 4
  };

  /* readonly attribute nsIPrintSettings currentPrintSettings; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCurrentPrintSettings(nsIPrintSettings **aCurrentPrintSettings) = 0;

  /* readonly attribute boolean doingPrint; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDoingPrint(bool *aDoingPrint) = 0;

  /* readonly attribute boolean doingPrintPreview; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDoingPrintPreview(bool *aDoingPrintPreview) = 0;

  /* readonly attribute long rawNumPages; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetRawNumPages(int32_t *aRawNumPages) = 0;

  /* readonly attribute long printPreviewNumPages; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPrintPreviewNumPages(int32_t *aPrintPreviewNumPages) = 0;

  /* readonly attribute long printPreviewCurrentPageNumber; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPrintPreviewCurrentPageNumber(int32_t *aPrintPreviewCurrentPageNumber) = 0;

  /* [noscript] void print (in nsIPrintSettings aThePrintSettings, in nsIWebProgressListener aWPListener); */
  NS_IMETHOD Print(nsIPrintSettings *aThePrintSettings, nsIWebProgressListener *aWPListener) = 0;

  /* [noscript] void printPreview (in nsIPrintSettings aThePrintSettings, in nsIWebProgressListener aWPListener, in PrintPreviewResolver aCallback); */
  NS_IMETHOD PrintPreview(nsIPrintSettings *aThePrintSettings, nsIWebProgressListener *aWPListener, std::function<void(const mozilla::dom::PrintPreviewResultInfo&)>&& aCallback) = 0;

  /* void printPreviewScrollToPage (in short aNavType, in long aPageNum); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD PrintPreviewScrollToPage(int16_t aNavType, int32_t aPageNum) = 0;

  /* void cancel (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Cancel(void) = 0;

  /* void exitPrintPreview (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ExitPrintPreview(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIWebBrowserPrint, NS_IWEBBROWSERPRINT_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIWEBBROWSERPRINT \
  NS_IMETHOD GetCurrentPrintSettings(nsIPrintSettings **aCurrentPrintSettings) override; \
  NS_IMETHOD GetDoingPrint(bool *aDoingPrint) override; \
  NS_IMETHOD GetDoingPrintPreview(bool *aDoingPrintPreview) override; \
  NS_IMETHOD GetRawNumPages(int32_t *aRawNumPages) override; \
  NS_IMETHOD GetPrintPreviewNumPages(int32_t *aPrintPreviewNumPages) override; \
  NS_IMETHOD GetPrintPreviewCurrentPageNumber(int32_t *aPrintPreviewCurrentPageNumber) override; \
  NS_IMETHOD Print(nsIPrintSettings *aThePrintSettings, nsIWebProgressListener *aWPListener) override; \
  NS_IMETHOD PrintPreview(nsIPrintSettings *aThePrintSettings, nsIWebProgressListener *aWPListener, std::function<void(const mozilla::dom::PrintPreviewResultInfo&)>&& aCallback) override; \
  NS_IMETHOD PrintPreviewScrollToPage(int16_t aNavType, int32_t aPageNum) override; \
  NS_IMETHOD Cancel(void) override; \
  NS_IMETHOD ExitPrintPreview(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIWEBBROWSERPRINT \
  nsresult GetCurrentPrintSettings(nsIPrintSettings **aCurrentPrintSettings); \
  nsresult GetDoingPrint(bool *aDoingPrint); \
  nsresult GetDoingPrintPreview(bool *aDoingPrintPreview); \
  nsresult GetRawNumPages(int32_t *aRawNumPages); \
  nsresult GetPrintPreviewNumPages(int32_t *aPrintPreviewNumPages); \
  nsresult GetPrintPreviewCurrentPageNumber(int32_t *aPrintPreviewCurrentPageNumber); \
  nsresult Print(nsIPrintSettings *aThePrintSettings, nsIWebProgressListener *aWPListener); \
  nsresult PrintPreview(nsIPrintSettings *aThePrintSettings, nsIWebProgressListener *aWPListener, std::function<void(const mozilla::dom::PrintPreviewResultInfo&)>&& aCallback); \
  nsresult PrintPreviewScrollToPage(int16_t aNavType, int32_t aPageNum); \
  nsresult Cancel(void); \
  nsresult ExitPrintPreview(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIWEBBROWSERPRINT(_to) \
  NS_IMETHOD GetCurrentPrintSettings(nsIPrintSettings **aCurrentPrintSettings) override { return _to GetCurrentPrintSettings(aCurrentPrintSettings); } \
  NS_IMETHOD GetDoingPrint(bool *aDoingPrint) override { return _to GetDoingPrint(aDoingPrint); } \
  NS_IMETHOD GetDoingPrintPreview(bool *aDoingPrintPreview) override { return _to GetDoingPrintPreview(aDoingPrintPreview); } \
  NS_IMETHOD GetRawNumPages(int32_t *aRawNumPages) override { return _to GetRawNumPages(aRawNumPages); } \
  NS_IMETHOD GetPrintPreviewNumPages(int32_t *aPrintPreviewNumPages) override { return _to GetPrintPreviewNumPages(aPrintPreviewNumPages); } \
  NS_IMETHOD GetPrintPreviewCurrentPageNumber(int32_t *aPrintPreviewCurrentPageNumber) override { return _to GetPrintPreviewCurrentPageNumber(aPrintPreviewCurrentPageNumber); } \
  NS_IMETHOD Print(nsIPrintSettings *aThePrintSettings, nsIWebProgressListener *aWPListener) override { return _to Print(aThePrintSettings, aWPListener); } \
  NS_IMETHOD PrintPreview(nsIPrintSettings *aThePrintSettings, nsIWebProgressListener *aWPListener, std::function<void(const mozilla::dom::PrintPreviewResultInfo&)>&& aCallback) override { return _to PrintPreview(aThePrintSettings, aWPListener, aCallback); } \
  NS_IMETHOD PrintPreviewScrollToPage(int16_t aNavType, int32_t aPageNum) override { return _to PrintPreviewScrollToPage(aNavType, aPageNum); } \
  NS_IMETHOD Cancel(void) override { return _to Cancel(); } \
  NS_IMETHOD ExitPrintPreview(void) override { return _to ExitPrintPreview(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIWEBBROWSERPRINT(_to) \
  NS_IMETHOD GetCurrentPrintSettings(nsIPrintSettings **aCurrentPrintSettings) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCurrentPrintSettings(aCurrentPrintSettings); } \
  NS_IMETHOD GetDoingPrint(bool *aDoingPrint) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDoingPrint(aDoingPrint); } \
  NS_IMETHOD GetDoingPrintPreview(bool *aDoingPrintPreview) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDoingPrintPreview(aDoingPrintPreview); } \
  NS_IMETHOD GetRawNumPages(int32_t *aRawNumPages) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRawNumPages(aRawNumPages); } \
  NS_IMETHOD GetPrintPreviewNumPages(int32_t *aPrintPreviewNumPages) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPrintPreviewNumPages(aPrintPreviewNumPages); } \
  NS_IMETHOD GetPrintPreviewCurrentPageNumber(int32_t *aPrintPreviewCurrentPageNumber) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPrintPreviewCurrentPageNumber(aPrintPreviewCurrentPageNumber); } \
  NS_IMETHOD Print(nsIPrintSettings *aThePrintSettings, nsIWebProgressListener *aWPListener) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Print(aThePrintSettings, aWPListener); } \
  NS_IMETHOD PrintPreview(nsIPrintSettings *aThePrintSettings, nsIWebProgressListener *aWPListener, std::function<void(const mozilla::dom::PrintPreviewResultInfo&)>&& aCallback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->PrintPreview(aThePrintSettings, aWPListener, aCallback); } \
  NS_IMETHOD PrintPreviewScrollToPage(int16_t aNavType, int32_t aPageNum) override { return !_to ? NS_ERROR_NULL_POINTER : _to->PrintPreviewScrollToPage(aNavType, aPageNum); } \
  NS_IMETHOD Cancel(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Cancel(); } \
  NS_IMETHOD ExitPrintPreview(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ExitPrintPreview(); } 


#endif /* __gen_nsIWebBrowserPrint_h__ */
