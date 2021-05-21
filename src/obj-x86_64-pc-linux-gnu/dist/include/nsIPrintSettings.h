/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/widget/nsIPrintSettings.idl
 */

#ifndef __gen_nsIPrintSettings_h__
#define __gen_nsIPrintSettings_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsTArray_h__
#include "nsTArray.h"
#endif

#include "mozilla/AlreadyAddRefed.h"
#include "mozilla/Assertions.h"
#include "mozilla/DebugOnly.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
#include "nsMargin.h"
#include "nsTArray.h"
class nsIPrintSession; /* forward declaration */


/* starting interface:    nsIPrintSettings */
#define NS_IPRINTSETTINGS_IID_STR "ecc5cbad-57fc-4731-b0bd-09e865bd62ad"

#define NS_IPRINTSETTINGS_IID \
  {0xecc5cbad, 0x57fc, 0x4731, \
    { 0xb0, 0xbd, 0x09, 0xe8, 0x65, 0xbd, 0x62, 0xad }}

class nsIPrintSettings : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPRINTSETTINGS_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPrintSettings;

  enum {
    kInitSaveHeaderLeft = 2U,
    kInitSaveHeaderCenter = 4U,
    kInitSaveHeaderRight = 8U,
    kInitSaveFooterLeft = 16U,
    kInitSaveFooterCenter = 32U,
    kInitSaveFooterRight = 64U,
    kInitSaveBGColors = 128U,
    kInitSaveBGImages = 256U,
    kInitSavePaperSize = 512U,
    kInitSaveResolution = 1024U,
    kInitSaveDuplex = 2048U,
    kInitSaveUnwriteableMargins = 16384U,
    kInitSaveEdges = 32768U,
    kInitSaveReversed = 65536U,
    kInitSaveInColor = 131072U,
    kInitSaveOrientation = 262144U,
    kInitSavePrinterName = 1048576U,
    kInitSavePrintToFile = 2097152U,
    kInitSaveToFileName = 4194304U,
    kInitSavePageDelay = 8388608U,
    kInitSaveMargins = 16777216U,
    kInitSaveNativeData = 33554432U,
    kInitSaveShrinkToFit = 134217728U,
    kInitSaveScaling = 268435456U,
    kInitSaveAll = 4294967295U,
    kGlobalSettings = 134447614U,
    kJustLeft = 0,
    kJustCenter = 1,
    kJustRight = 2,
    kPaperSizeInches = 0,
    kPaperSizeMillimeters = 1,
    kPortraitOrientation = 0,
    kLandscapeOrientation = 1,
    kOutputFormatNative = 0,
    kOutputFormatPS = 1,
    kOutputFormatPDF = 2,
    kDuplexNone = 0,
    kDuplexFlipOnLongEdge = 1,
    kDuplexFlipOnShortEdge = 2
  };

  /* void GetEffectivePageSize (out double aWidth, out double aHeight); */
  NS_IMETHOD GetEffectivePageSize(double *aWidth, double *aHeight) = 0;

  /* [noscript,nostdcall,notxpcom] void GetEffectiveSheetSize (out double aWidth, out double aHeight); */
  virtual void GetEffectiveSheetSize(double *aWidth, double *aHeight) = 0;

  /* [noscript,nostdcall,notxpcom] long GetSheetOrientation (); */
  virtual int32_t GetSheetOrientation(void) = 0;

  /* [noscript,nostdcall,notxpcom] bool HasOrthogonalSheetsAndPages (); */
  virtual bool HasOrthogonalSheetsAndPages(void) = 0;

  /* nsIPrintSettings clone (); */
  NS_IMETHOD Clone(nsIPrintSettings **_retval) = 0;

  /* void assign (in nsIPrintSettings aPS); */
  NS_IMETHOD Assign(nsIPrintSettings *aPS) = 0;

  /* [noscript] attribute nsIPrintSession printSession; */
  NS_IMETHOD GetPrintSession(nsIPrintSession **aPrintSession) = 0;
  NS_IMETHOD SetPrintSession(nsIPrintSession *aPrintSession) = 0;

  /* attribute double edgeTop; */
  NS_IMETHOD GetEdgeTop(double *aEdgeTop) = 0;
  NS_IMETHOD SetEdgeTop(double aEdgeTop) = 0;

  /* attribute double edgeLeft; */
  NS_IMETHOD GetEdgeLeft(double *aEdgeLeft) = 0;
  NS_IMETHOD SetEdgeLeft(double aEdgeLeft) = 0;

  /* attribute double edgeBottom; */
  NS_IMETHOD GetEdgeBottom(double *aEdgeBottom) = 0;
  NS_IMETHOD SetEdgeBottom(double aEdgeBottom) = 0;

  /* attribute double edgeRight; */
  NS_IMETHOD GetEdgeRight(double *aEdgeRight) = 0;
  NS_IMETHOD SetEdgeRight(double aEdgeRight) = 0;

  /* attribute double marginTop; */
  NS_IMETHOD GetMarginTop(double *aMarginTop) = 0;
  NS_IMETHOD SetMarginTop(double aMarginTop) = 0;

  /* attribute double marginLeft; */
  NS_IMETHOD GetMarginLeft(double *aMarginLeft) = 0;
  NS_IMETHOD SetMarginLeft(double aMarginLeft) = 0;

  /* attribute double marginBottom; */
  NS_IMETHOD GetMarginBottom(double *aMarginBottom) = 0;
  NS_IMETHOD SetMarginBottom(double aMarginBottom) = 0;

  /* attribute double marginRight; */
  NS_IMETHOD GetMarginRight(double *aMarginRight) = 0;
  NS_IMETHOD SetMarginRight(double aMarginRight) = 0;

  /* attribute double unwriteableMarginTop; */
  NS_IMETHOD GetUnwriteableMarginTop(double *aUnwriteableMarginTop) = 0;
  NS_IMETHOD SetUnwriteableMarginTop(double aUnwriteableMarginTop) = 0;

  /* attribute double unwriteableMarginLeft; */
  NS_IMETHOD GetUnwriteableMarginLeft(double *aUnwriteableMarginLeft) = 0;
  NS_IMETHOD SetUnwriteableMarginLeft(double aUnwriteableMarginLeft) = 0;

  /* attribute double unwriteableMarginBottom; */
  NS_IMETHOD GetUnwriteableMarginBottom(double *aUnwriteableMarginBottom) = 0;
  NS_IMETHOD SetUnwriteableMarginBottom(double aUnwriteableMarginBottom) = 0;

  /* attribute double unwriteableMarginRight; */
  NS_IMETHOD GetUnwriteableMarginRight(double *aUnwriteableMarginRight) = 0;
  NS_IMETHOD SetUnwriteableMarginRight(double aUnwriteableMarginRight) = 0;

  /* attribute double scaling; */
  NS_IMETHOD GetScaling(double *aScaling) = 0;
  NS_IMETHOD SetScaling(double aScaling) = 0;

  /* [infallible] attribute boolean printBGColors; */
  NS_IMETHOD GetPrintBGColors(bool *aPrintBGColors) = 0;
  inline bool  GetPrintBGColors()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetPrintBGColors(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }
  NS_IMETHOD SetPrintBGColors(bool aPrintBGColors) = 0;

  /* [infallible] attribute boolean printBGImages; */
  NS_IMETHOD GetPrintBGImages(bool *aPrintBGImages) = 0;
  inline bool  GetPrintBGImages()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetPrintBGImages(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }
  NS_IMETHOD SetPrintBGImages(bool aPrintBGImages) = 0;

  /* [infallible] attribute boolean honorPageRuleMargins; */
  NS_IMETHOD GetHonorPageRuleMargins(bool *aHonorPageRuleMargins) = 0;
  inline bool  GetHonorPageRuleMargins()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetHonorPageRuleMargins(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }
  NS_IMETHOD SetHonorPageRuleMargins(bool aHonorPageRuleMargins) = 0;

  /* [infallible] attribute boolean showMarginGuides; */
  NS_IMETHOD GetShowMarginGuides(bool *aShowMarginGuides) = 0;
  inline bool  GetShowMarginGuides()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetShowMarginGuides(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }
  NS_IMETHOD SetShowMarginGuides(bool aShowMarginGuides) = 0;

  /* [infallible] attribute boolean isPrintSelectionRBEnabled; */
  NS_IMETHOD GetIsPrintSelectionRBEnabled(bool *aIsPrintSelectionRBEnabled) = 0;
  inline bool  GetIsPrintSelectionRBEnabled()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetIsPrintSelectionRBEnabled(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }
  NS_IMETHOD SetIsPrintSelectionRBEnabled(bool aIsPrintSelectionRBEnabled) = 0;

  /* [infallible] attribute boolean printSelectionOnly; */
  NS_IMETHOD GetPrintSelectionOnly(bool *aPrintSelectionOnly) = 0;
  inline bool  GetPrintSelectionOnly()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetPrintSelectionOnly(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }
  NS_IMETHOD SetPrintSelectionOnly(bool aPrintSelectionOnly) = 0;

  /* attribute AString title; */
  NS_IMETHOD GetTitle(nsAString& aTitle) = 0;
  NS_IMETHOD SetTitle(const nsAString& aTitle) = 0;

  /* attribute AString docURL; */
  NS_IMETHOD GetDocURL(nsAString& aDocURL) = 0;
  NS_IMETHOD SetDocURL(const nsAString& aDocURL) = 0;

  /* attribute AString headerStrLeft; */
  NS_IMETHOD GetHeaderStrLeft(nsAString& aHeaderStrLeft) = 0;
  NS_IMETHOD SetHeaderStrLeft(const nsAString& aHeaderStrLeft) = 0;

  /* attribute AString headerStrCenter; */
  NS_IMETHOD GetHeaderStrCenter(nsAString& aHeaderStrCenter) = 0;
  NS_IMETHOD SetHeaderStrCenter(const nsAString& aHeaderStrCenter) = 0;

  /* attribute AString headerStrRight; */
  NS_IMETHOD GetHeaderStrRight(nsAString& aHeaderStrRight) = 0;
  NS_IMETHOD SetHeaderStrRight(const nsAString& aHeaderStrRight) = 0;

  /* attribute AString footerStrLeft; */
  NS_IMETHOD GetFooterStrLeft(nsAString& aFooterStrLeft) = 0;
  NS_IMETHOD SetFooterStrLeft(const nsAString& aFooterStrLeft) = 0;

  /* attribute AString footerStrCenter; */
  NS_IMETHOD GetFooterStrCenter(nsAString& aFooterStrCenter) = 0;
  NS_IMETHOD SetFooterStrCenter(const nsAString& aFooterStrCenter) = 0;

  /* attribute AString footerStrRight; */
  NS_IMETHOD GetFooterStrRight(nsAString& aFooterStrRight) = 0;
  NS_IMETHOD SetFooterStrRight(const nsAString& aFooterStrRight) = 0;

  /* attribute boolean isCancelled; */
  NS_IMETHOD GetIsCancelled(bool *aIsCancelled) = 0;
  NS_IMETHOD SetIsCancelled(bool aIsCancelled) = 0;

  /* readonly attribute boolean saveOnCancel; */
  NS_IMETHOD GetSaveOnCancel(bool *aSaveOnCancel) = 0;

  /* attribute boolean printSilent; */
  NS_IMETHOD GetPrintSilent(bool *aPrintSilent) = 0;
  NS_IMETHOD SetPrintSilent(bool aPrintSilent) = 0;

  /* attribute boolean shrinkToFit; */
  NS_IMETHOD GetShrinkToFit(bool *aShrinkToFit) = 0;
  NS_IMETHOD SetShrinkToFit(bool aShrinkToFit) = 0;

  /* attribute boolean showPrintProgress; */
  NS_IMETHOD GetShowPrintProgress(bool *aShowPrintProgress) = 0;
  NS_IMETHOD SetShowPrintProgress(bool aShowPrintProgress) = 0;

  /* attribute AString paperId; */
  NS_IMETHOD GetPaperId(nsAString& aPaperId) = 0;
  NS_IMETHOD SetPaperId(const nsAString& aPaperId) = 0;

  /* attribute double paperWidth; */
  NS_IMETHOD GetPaperWidth(double *aPaperWidth) = 0;
  NS_IMETHOD SetPaperWidth(double aPaperWidth) = 0;

  /* attribute double paperHeight; */
  NS_IMETHOD GetPaperHeight(double *aPaperHeight) = 0;
  NS_IMETHOD SetPaperHeight(double aPaperHeight) = 0;

  /* attribute short paperSizeUnit; */
  NS_IMETHOD GetPaperSizeUnit(int16_t *aPaperSizeUnit) = 0;
  NS_IMETHOD SetPaperSizeUnit(int16_t aPaperSizeUnit) = 0;

  /* attribute boolean printReversed; */
  NS_IMETHOD GetPrintReversed(bool *aPrintReversed) = 0;
  NS_IMETHOD SetPrintReversed(bool aPrintReversed) = 0;

  /* [infallible] attribute boolean printInColor; */
  NS_IMETHOD GetPrintInColor(bool *aPrintInColor) = 0;
  inline bool  GetPrintInColor()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetPrintInColor(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }
  NS_IMETHOD SetPrintInColor(bool aPrintInColor) = 0;

  /* attribute long orientation; */
  NS_IMETHOD GetOrientation(int32_t *aOrientation) = 0;
  NS_IMETHOD SetOrientation(int32_t aOrientation) = 0;

  /* attribute long numCopies; */
  NS_IMETHOD GetNumCopies(int32_t *aNumCopies) = 0;
  NS_IMETHOD SetNumCopies(int32_t aNumCopies) = 0;

  /* attribute long numPagesPerSheet; */
  NS_IMETHOD GetNumPagesPerSheet(int32_t *aNumPagesPerSheet) = 0;
  NS_IMETHOD SetNumPagesPerSheet(int32_t aNumPagesPerSheet) = 0;

  /* attribute AString printerName; */
  NS_IMETHOD GetPrinterName(nsAString& aPrinterName) = 0;
  NS_IMETHOD SetPrinterName(const nsAString& aPrinterName) = 0;

  /* attribute boolean printToFile; */
  NS_IMETHOD GetPrintToFile(bool *aPrintToFile) = 0;
  NS_IMETHOD SetPrintToFile(bool aPrintToFile) = 0;

  /* attribute AString toFileName; */
  NS_IMETHOD GetToFileName(nsAString& aToFileName) = 0;
  NS_IMETHOD SetToFileName(const nsAString& aToFileName) = 0;

  /* attribute short outputFormat; */
  NS_IMETHOD GetOutputFormat(int16_t *aOutputFormat) = 0;
  NS_IMETHOD SetOutputFormat(int16_t aOutputFormat) = 0;

  /* attribute long printPageDelay; */
  NS_IMETHOD GetPrintPageDelay(int32_t *aPrintPageDelay) = 0;
  NS_IMETHOD SetPrintPageDelay(int32_t aPrintPageDelay) = 0;

  /* attribute long resolution; */
  NS_IMETHOD GetResolution(int32_t *aResolution) = 0;
  NS_IMETHOD SetResolution(int32_t aResolution) = 0;

  /* attribute long duplex; */
  NS_IMETHOD GetDuplex(int32_t *aDuplex) = 0;
  NS_IMETHOD SetDuplex(int32_t aDuplex) = 0;

  /* attribute boolean isInitializedFromPrinter; */
  NS_IMETHOD GetIsInitializedFromPrinter(bool *aIsInitializedFromPrinter) = 0;
  NS_IMETHOD SetIsInitializedFromPrinter(bool aIsInitializedFromPrinter) = 0;

  /* attribute boolean isInitializedFromPrefs; */
  NS_IMETHOD GetIsInitializedFromPrefs(bool *aIsInitializedFromPrefs) = 0;
  NS_IMETHOD SetIsInitializedFromPrefs(bool aIsInitializedFromPrefs) = 0;

  /* [noscript] void SetMarginInTwips (in nsNativeIntMarginRef aMargin); */
  NS_IMETHOD SetMarginInTwips(nsIntMargin & aMargin) = 0;

  /* [noscript] void SetEdgeInTwips (in nsNativeIntMarginRef aEdge); */
  NS_IMETHOD SetEdgeInTwips(nsIntMargin & aEdge) = 0;

  /* [noscript,nostdcall,notxpcom] nsNativeIntMargin GetMarginInTwips (); */
  virtual nsIntMargin GetMarginInTwips(void) = 0;

  /* [noscript,nostdcall,notxpcom] nsNativeIntMargin GetEdgeInTwips (); */
  virtual nsIntMargin GetEdgeInTwips(void) = 0;

  /* [noscript] void SetupSilentPrinting (); */
  NS_IMETHOD SetupSilentPrinting(void) = 0;

  /* [noscript] void SetUnwriteableMarginInTwips (in nsNativeIntMarginRef aEdge); */
  NS_IMETHOD SetUnwriteableMarginInTwips(nsIntMargin & aEdge) = 0;

  /* [noscript,nostdcall,notxpcom] nsNativeIntMargin GetUnwriteableMarginInTwips (); */
  virtual nsIntMargin GetUnwriteableMarginInTwips(void) = 0;

  /* attribute Array<long> pageRanges; */
  NS_IMETHOD GetPageRanges(nsTArray<int32_t >& aPageRanges) = 0;
  NS_IMETHOD SetPageRanges(const nsTArray<int32_t >& aPageRanges) = 0;

   static bool IsPageSkipped(int32_t aPageNum, const nsTArray<int32_t>& aRanges);
};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPrintSettings, NS_IPRINTSETTINGS_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPRINTSETTINGS \
  NS_IMETHOD GetEffectivePageSize(double *aWidth, double *aHeight) override; \
  virtual void GetEffectiveSheetSize(double *aWidth, double *aHeight) override; \
  virtual int32_t GetSheetOrientation(void) override; \
  virtual bool HasOrthogonalSheetsAndPages(void) override; \
  NS_IMETHOD Clone(nsIPrintSettings **_retval) override; \
  NS_IMETHOD Assign(nsIPrintSettings *aPS) override; \
  NS_IMETHOD GetPrintSession(nsIPrintSession **aPrintSession) override; \
  NS_IMETHOD SetPrintSession(nsIPrintSession *aPrintSession) override; \
  NS_IMETHOD GetEdgeTop(double *aEdgeTop) override; \
  NS_IMETHOD SetEdgeTop(double aEdgeTop) override; \
  NS_IMETHOD GetEdgeLeft(double *aEdgeLeft) override; \
  NS_IMETHOD SetEdgeLeft(double aEdgeLeft) override; \
  NS_IMETHOD GetEdgeBottom(double *aEdgeBottom) override; \
  NS_IMETHOD SetEdgeBottom(double aEdgeBottom) override; \
  NS_IMETHOD GetEdgeRight(double *aEdgeRight) override; \
  NS_IMETHOD SetEdgeRight(double aEdgeRight) override; \
  NS_IMETHOD GetMarginTop(double *aMarginTop) override; \
  NS_IMETHOD SetMarginTop(double aMarginTop) override; \
  NS_IMETHOD GetMarginLeft(double *aMarginLeft) override; \
  NS_IMETHOD SetMarginLeft(double aMarginLeft) override; \
  NS_IMETHOD GetMarginBottom(double *aMarginBottom) override; \
  NS_IMETHOD SetMarginBottom(double aMarginBottom) override; \
  NS_IMETHOD GetMarginRight(double *aMarginRight) override; \
  NS_IMETHOD SetMarginRight(double aMarginRight) override; \
  NS_IMETHOD GetUnwriteableMarginTop(double *aUnwriteableMarginTop) override; \
  NS_IMETHOD SetUnwriteableMarginTop(double aUnwriteableMarginTop) override; \
  NS_IMETHOD GetUnwriteableMarginLeft(double *aUnwriteableMarginLeft) override; \
  NS_IMETHOD SetUnwriteableMarginLeft(double aUnwriteableMarginLeft) override; \
  NS_IMETHOD GetUnwriteableMarginBottom(double *aUnwriteableMarginBottom) override; \
  NS_IMETHOD SetUnwriteableMarginBottom(double aUnwriteableMarginBottom) override; \
  NS_IMETHOD GetUnwriteableMarginRight(double *aUnwriteableMarginRight) override; \
  NS_IMETHOD SetUnwriteableMarginRight(double aUnwriteableMarginRight) override; \
  NS_IMETHOD GetScaling(double *aScaling) override; \
  NS_IMETHOD SetScaling(double aScaling) override; \
  using nsIPrintSettings::GetPrintBGColors; \
  NS_IMETHOD GetPrintBGColors(bool *aPrintBGColors) override; \
  NS_IMETHOD SetPrintBGColors(bool aPrintBGColors) override; \
  using nsIPrintSettings::GetPrintBGImages; \
  NS_IMETHOD GetPrintBGImages(bool *aPrintBGImages) override; \
  NS_IMETHOD SetPrintBGImages(bool aPrintBGImages) override; \
  using nsIPrintSettings::GetHonorPageRuleMargins; \
  NS_IMETHOD GetHonorPageRuleMargins(bool *aHonorPageRuleMargins) override; \
  NS_IMETHOD SetHonorPageRuleMargins(bool aHonorPageRuleMargins) override; \
  using nsIPrintSettings::GetShowMarginGuides; \
  NS_IMETHOD GetShowMarginGuides(bool *aShowMarginGuides) override; \
  NS_IMETHOD SetShowMarginGuides(bool aShowMarginGuides) override; \
  using nsIPrintSettings::GetIsPrintSelectionRBEnabled; \
  NS_IMETHOD GetIsPrintSelectionRBEnabled(bool *aIsPrintSelectionRBEnabled) override; \
  NS_IMETHOD SetIsPrintSelectionRBEnabled(bool aIsPrintSelectionRBEnabled) override; \
  using nsIPrintSettings::GetPrintSelectionOnly; \
  NS_IMETHOD GetPrintSelectionOnly(bool *aPrintSelectionOnly) override; \
  NS_IMETHOD SetPrintSelectionOnly(bool aPrintSelectionOnly) override; \
  NS_IMETHOD GetTitle(nsAString& aTitle) override; \
  NS_IMETHOD SetTitle(const nsAString& aTitle) override; \
  NS_IMETHOD GetDocURL(nsAString& aDocURL) override; \
  NS_IMETHOD SetDocURL(const nsAString& aDocURL) override; \
  NS_IMETHOD GetHeaderStrLeft(nsAString& aHeaderStrLeft) override; \
  NS_IMETHOD SetHeaderStrLeft(const nsAString& aHeaderStrLeft) override; \
  NS_IMETHOD GetHeaderStrCenter(nsAString& aHeaderStrCenter) override; \
  NS_IMETHOD SetHeaderStrCenter(const nsAString& aHeaderStrCenter) override; \
  NS_IMETHOD GetHeaderStrRight(nsAString& aHeaderStrRight) override; \
  NS_IMETHOD SetHeaderStrRight(const nsAString& aHeaderStrRight) override; \
  NS_IMETHOD GetFooterStrLeft(nsAString& aFooterStrLeft) override; \
  NS_IMETHOD SetFooterStrLeft(const nsAString& aFooterStrLeft) override; \
  NS_IMETHOD GetFooterStrCenter(nsAString& aFooterStrCenter) override; \
  NS_IMETHOD SetFooterStrCenter(const nsAString& aFooterStrCenter) override; \
  NS_IMETHOD GetFooterStrRight(nsAString& aFooterStrRight) override; \
  NS_IMETHOD SetFooterStrRight(const nsAString& aFooterStrRight) override; \
  NS_IMETHOD GetIsCancelled(bool *aIsCancelled) override; \
  NS_IMETHOD SetIsCancelled(bool aIsCancelled) override; \
  NS_IMETHOD GetSaveOnCancel(bool *aSaveOnCancel) override; \
  NS_IMETHOD GetPrintSilent(bool *aPrintSilent) override; \
  NS_IMETHOD SetPrintSilent(bool aPrintSilent) override; \
  NS_IMETHOD GetShrinkToFit(bool *aShrinkToFit) override; \
  NS_IMETHOD SetShrinkToFit(bool aShrinkToFit) override; \
  NS_IMETHOD GetShowPrintProgress(bool *aShowPrintProgress) override; \
  NS_IMETHOD SetShowPrintProgress(bool aShowPrintProgress) override; \
  NS_IMETHOD GetPaperId(nsAString& aPaperId) override; \
  NS_IMETHOD SetPaperId(const nsAString& aPaperId) override; \
  NS_IMETHOD GetPaperWidth(double *aPaperWidth) override; \
  NS_IMETHOD SetPaperWidth(double aPaperWidth) override; \
  NS_IMETHOD GetPaperHeight(double *aPaperHeight) override; \
  NS_IMETHOD SetPaperHeight(double aPaperHeight) override; \
  NS_IMETHOD GetPaperSizeUnit(int16_t *aPaperSizeUnit) override; \
  NS_IMETHOD SetPaperSizeUnit(int16_t aPaperSizeUnit) override; \
  NS_IMETHOD GetPrintReversed(bool *aPrintReversed) override; \
  NS_IMETHOD SetPrintReversed(bool aPrintReversed) override; \
  using nsIPrintSettings::GetPrintInColor; \
  NS_IMETHOD GetPrintInColor(bool *aPrintInColor) override; \
  NS_IMETHOD SetPrintInColor(bool aPrintInColor) override; \
  NS_IMETHOD GetOrientation(int32_t *aOrientation) override; \
  NS_IMETHOD SetOrientation(int32_t aOrientation) override; \
  NS_IMETHOD GetNumCopies(int32_t *aNumCopies) override; \
  NS_IMETHOD SetNumCopies(int32_t aNumCopies) override; \
  NS_IMETHOD GetNumPagesPerSheet(int32_t *aNumPagesPerSheet) override; \
  NS_IMETHOD SetNumPagesPerSheet(int32_t aNumPagesPerSheet) override; \
  NS_IMETHOD GetPrinterName(nsAString& aPrinterName) override; \
  NS_IMETHOD SetPrinterName(const nsAString& aPrinterName) override; \
  NS_IMETHOD GetPrintToFile(bool *aPrintToFile) override; \
  NS_IMETHOD SetPrintToFile(bool aPrintToFile) override; \
  NS_IMETHOD GetToFileName(nsAString& aToFileName) override; \
  NS_IMETHOD SetToFileName(const nsAString& aToFileName) override; \
  NS_IMETHOD GetOutputFormat(int16_t *aOutputFormat) override; \
  NS_IMETHOD SetOutputFormat(int16_t aOutputFormat) override; \
  NS_IMETHOD GetPrintPageDelay(int32_t *aPrintPageDelay) override; \
  NS_IMETHOD SetPrintPageDelay(int32_t aPrintPageDelay) override; \
  NS_IMETHOD GetResolution(int32_t *aResolution) override; \
  NS_IMETHOD SetResolution(int32_t aResolution) override; \
  NS_IMETHOD GetDuplex(int32_t *aDuplex) override; \
  NS_IMETHOD SetDuplex(int32_t aDuplex) override; \
  NS_IMETHOD GetIsInitializedFromPrinter(bool *aIsInitializedFromPrinter) override; \
  NS_IMETHOD SetIsInitializedFromPrinter(bool aIsInitializedFromPrinter) override; \
  NS_IMETHOD GetIsInitializedFromPrefs(bool *aIsInitializedFromPrefs) override; \
  NS_IMETHOD SetIsInitializedFromPrefs(bool aIsInitializedFromPrefs) override; \
  NS_IMETHOD SetMarginInTwips(nsIntMargin & aMargin) override; \
  NS_IMETHOD SetEdgeInTwips(nsIntMargin & aEdge) override; \
  virtual nsIntMargin GetMarginInTwips(void) override; \
  virtual nsIntMargin GetEdgeInTwips(void) override; \
  NS_IMETHOD SetupSilentPrinting(void) override; \
  NS_IMETHOD SetUnwriteableMarginInTwips(nsIntMargin & aEdge) override; \
  virtual nsIntMargin GetUnwriteableMarginInTwips(void) override; \
  NS_IMETHOD GetPageRanges(nsTArray<int32_t >& aPageRanges) override; \
  NS_IMETHOD SetPageRanges(const nsTArray<int32_t >& aPageRanges) override; \

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPRINTSETTINGS \
  nsresult GetEffectivePageSize(double *aWidth, double *aHeight); \
  void GetEffectiveSheetSize(double *aWidth, double *aHeight); \
  int32_t GetSheetOrientation(void); \
  bool HasOrthogonalSheetsAndPages(void); \
  nsresult Clone(nsIPrintSettings **_retval); \
  nsresult Assign(nsIPrintSettings *aPS); \
  nsresult GetPrintSession(nsIPrintSession **aPrintSession); \
  nsresult SetPrintSession(nsIPrintSession *aPrintSession); \
  nsresult GetEdgeTop(double *aEdgeTop); \
  nsresult SetEdgeTop(double aEdgeTop); \
  nsresult GetEdgeLeft(double *aEdgeLeft); \
  nsresult SetEdgeLeft(double aEdgeLeft); \
  nsresult GetEdgeBottom(double *aEdgeBottom); \
  nsresult SetEdgeBottom(double aEdgeBottom); \
  nsresult GetEdgeRight(double *aEdgeRight); \
  nsresult SetEdgeRight(double aEdgeRight); \
  nsresult GetMarginTop(double *aMarginTop); \
  nsresult SetMarginTop(double aMarginTop); \
  nsresult GetMarginLeft(double *aMarginLeft); \
  nsresult SetMarginLeft(double aMarginLeft); \
  nsresult GetMarginBottom(double *aMarginBottom); \
  nsresult SetMarginBottom(double aMarginBottom); \
  nsresult GetMarginRight(double *aMarginRight); \
  nsresult SetMarginRight(double aMarginRight); \
  nsresult GetUnwriteableMarginTop(double *aUnwriteableMarginTop); \
  nsresult SetUnwriteableMarginTop(double aUnwriteableMarginTop); \
  nsresult GetUnwriteableMarginLeft(double *aUnwriteableMarginLeft); \
  nsresult SetUnwriteableMarginLeft(double aUnwriteableMarginLeft); \
  nsresult GetUnwriteableMarginBottom(double *aUnwriteableMarginBottom); \
  nsresult SetUnwriteableMarginBottom(double aUnwriteableMarginBottom); \
  nsresult GetUnwriteableMarginRight(double *aUnwriteableMarginRight); \
  nsresult SetUnwriteableMarginRight(double aUnwriteableMarginRight); \
  nsresult GetScaling(double *aScaling); \
  nsresult SetScaling(double aScaling); \
  using nsIPrintSettings::GetPrintBGColors; \
  nsresult GetPrintBGColors(bool *aPrintBGColors); \
  nsresult SetPrintBGColors(bool aPrintBGColors); \
  using nsIPrintSettings::GetPrintBGImages; \
  nsresult GetPrintBGImages(bool *aPrintBGImages); \
  nsresult SetPrintBGImages(bool aPrintBGImages); \
  using nsIPrintSettings::GetHonorPageRuleMargins; \
  nsresult GetHonorPageRuleMargins(bool *aHonorPageRuleMargins); \
  nsresult SetHonorPageRuleMargins(bool aHonorPageRuleMargins); \
  using nsIPrintSettings::GetShowMarginGuides; \
  nsresult GetShowMarginGuides(bool *aShowMarginGuides); \
  nsresult SetShowMarginGuides(bool aShowMarginGuides); \
  using nsIPrintSettings::GetIsPrintSelectionRBEnabled; \
  nsresult GetIsPrintSelectionRBEnabled(bool *aIsPrintSelectionRBEnabled); \
  nsresult SetIsPrintSelectionRBEnabled(bool aIsPrintSelectionRBEnabled); \
  using nsIPrintSettings::GetPrintSelectionOnly; \
  nsresult GetPrintSelectionOnly(bool *aPrintSelectionOnly); \
  nsresult SetPrintSelectionOnly(bool aPrintSelectionOnly); \
  nsresult GetTitle(nsAString& aTitle); \
  nsresult SetTitle(const nsAString& aTitle); \
  nsresult GetDocURL(nsAString& aDocURL); \
  nsresult SetDocURL(const nsAString& aDocURL); \
  nsresult GetHeaderStrLeft(nsAString& aHeaderStrLeft); \
  nsresult SetHeaderStrLeft(const nsAString& aHeaderStrLeft); \
  nsresult GetHeaderStrCenter(nsAString& aHeaderStrCenter); \
  nsresult SetHeaderStrCenter(const nsAString& aHeaderStrCenter); \
  nsresult GetHeaderStrRight(nsAString& aHeaderStrRight); \
  nsresult SetHeaderStrRight(const nsAString& aHeaderStrRight); \
  nsresult GetFooterStrLeft(nsAString& aFooterStrLeft); \
  nsresult SetFooterStrLeft(const nsAString& aFooterStrLeft); \
  nsresult GetFooterStrCenter(nsAString& aFooterStrCenter); \
  nsresult SetFooterStrCenter(const nsAString& aFooterStrCenter); \
  nsresult GetFooterStrRight(nsAString& aFooterStrRight); \
  nsresult SetFooterStrRight(const nsAString& aFooterStrRight); \
  nsresult GetIsCancelled(bool *aIsCancelled); \
  nsresult SetIsCancelled(bool aIsCancelled); \
  nsresult GetSaveOnCancel(bool *aSaveOnCancel); \
  nsresult GetPrintSilent(bool *aPrintSilent); \
  nsresult SetPrintSilent(bool aPrintSilent); \
  nsresult GetShrinkToFit(bool *aShrinkToFit); \
  nsresult SetShrinkToFit(bool aShrinkToFit); \
  nsresult GetShowPrintProgress(bool *aShowPrintProgress); \
  nsresult SetShowPrintProgress(bool aShowPrintProgress); \
  nsresult GetPaperId(nsAString& aPaperId); \
  nsresult SetPaperId(const nsAString& aPaperId); \
  nsresult GetPaperWidth(double *aPaperWidth); \
  nsresult SetPaperWidth(double aPaperWidth); \
  nsresult GetPaperHeight(double *aPaperHeight); \
  nsresult SetPaperHeight(double aPaperHeight); \
  nsresult GetPaperSizeUnit(int16_t *aPaperSizeUnit); \
  nsresult SetPaperSizeUnit(int16_t aPaperSizeUnit); \
  nsresult GetPrintReversed(bool *aPrintReversed); \
  nsresult SetPrintReversed(bool aPrintReversed); \
  using nsIPrintSettings::GetPrintInColor; \
  nsresult GetPrintInColor(bool *aPrintInColor); \
  nsresult SetPrintInColor(bool aPrintInColor); \
  nsresult GetOrientation(int32_t *aOrientation); \
  nsresult SetOrientation(int32_t aOrientation); \
  nsresult GetNumCopies(int32_t *aNumCopies); \
  nsresult SetNumCopies(int32_t aNumCopies); \
  nsresult GetNumPagesPerSheet(int32_t *aNumPagesPerSheet); \
  nsresult SetNumPagesPerSheet(int32_t aNumPagesPerSheet); \
  nsresult GetPrinterName(nsAString& aPrinterName); \
  nsresult SetPrinterName(const nsAString& aPrinterName); \
  nsresult GetPrintToFile(bool *aPrintToFile); \
  nsresult SetPrintToFile(bool aPrintToFile); \
  nsresult GetToFileName(nsAString& aToFileName); \
  nsresult SetToFileName(const nsAString& aToFileName); \
  nsresult GetOutputFormat(int16_t *aOutputFormat); \
  nsresult SetOutputFormat(int16_t aOutputFormat); \
  nsresult GetPrintPageDelay(int32_t *aPrintPageDelay); \
  nsresult SetPrintPageDelay(int32_t aPrintPageDelay); \
  nsresult GetResolution(int32_t *aResolution); \
  nsresult SetResolution(int32_t aResolution); \
  nsresult GetDuplex(int32_t *aDuplex); \
  nsresult SetDuplex(int32_t aDuplex); \
  nsresult GetIsInitializedFromPrinter(bool *aIsInitializedFromPrinter); \
  nsresult SetIsInitializedFromPrinter(bool aIsInitializedFromPrinter); \
  nsresult GetIsInitializedFromPrefs(bool *aIsInitializedFromPrefs); \
  nsresult SetIsInitializedFromPrefs(bool aIsInitializedFromPrefs); \
  nsresult SetMarginInTwips(nsIntMargin & aMargin); \
  nsresult SetEdgeInTwips(nsIntMargin & aEdge); \
  nsIntMargin GetMarginInTwips(void); \
  nsIntMargin GetEdgeInTwips(void); \
  nsresult SetupSilentPrinting(void); \
  nsresult SetUnwriteableMarginInTwips(nsIntMargin & aEdge); \
  nsIntMargin GetUnwriteableMarginInTwips(void); \
  nsresult GetPageRanges(nsTArray<int32_t >& aPageRanges); \
  nsresult SetPageRanges(const nsTArray<int32_t >& aPageRanges); \

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPRINTSETTINGS(_to) \
  NS_IMETHOD GetEffectivePageSize(double *aWidth, double *aHeight) override { return _to GetEffectivePageSize(aWidth, aHeight); } \
  virtual void GetEffectiveSheetSize(double *aWidth, double *aHeight) override { return _to GetEffectiveSheetSize(aWidth, aHeight); } \
  virtual int32_t GetSheetOrientation(void) override { return _to GetSheetOrientation(); } \
  virtual bool HasOrthogonalSheetsAndPages(void) override { return _to HasOrthogonalSheetsAndPages(); } \
  NS_IMETHOD Clone(nsIPrintSettings **_retval) override { return _to Clone(_retval); } \
  NS_IMETHOD Assign(nsIPrintSettings *aPS) override { return _to Assign(aPS); } \
  NS_IMETHOD GetPrintSession(nsIPrintSession **aPrintSession) override { return _to GetPrintSession(aPrintSession); } \
  NS_IMETHOD SetPrintSession(nsIPrintSession *aPrintSession) override { return _to SetPrintSession(aPrintSession); } \
  NS_IMETHOD GetEdgeTop(double *aEdgeTop) override { return _to GetEdgeTop(aEdgeTop); } \
  NS_IMETHOD SetEdgeTop(double aEdgeTop) override { return _to SetEdgeTop(aEdgeTop); } \
  NS_IMETHOD GetEdgeLeft(double *aEdgeLeft) override { return _to GetEdgeLeft(aEdgeLeft); } \
  NS_IMETHOD SetEdgeLeft(double aEdgeLeft) override { return _to SetEdgeLeft(aEdgeLeft); } \
  NS_IMETHOD GetEdgeBottom(double *aEdgeBottom) override { return _to GetEdgeBottom(aEdgeBottom); } \
  NS_IMETHOD SetEdgeBottom(double aEdgeBottom) override { return _to SetEdgeBottom(aEdgeBottom); } \
  NS_IMETHOD GetEdgeRight(double *aEdgeRight) override { return _to GetEdgeRight(aEdgeRight); } \
  NS_IMETHOD SetEdgeRight(double aEdgeRight) override { return _to SetEdgeRight(aEdgeRight); } \
  NS_IMETHOD GetMarginTop(double *aMarginTop) override { return _to GetMarginTop(aMarginTop); } \
  NS_IMETHOD SetMarginTop(double aMarginTop) override { return _to SetMarginTop(aMarginTop); } \
  NS_IMETHOD GetMarginLeft(double *aMarginLeft) override { return _to GetMarginLeft(aMarginLeft); } \
  NS_IMETHOD SetMarginLeft(double aMarginLeft) override { return _to SetMarginLeft(aMarginLeft); } \
  NS_IMETHOD GetMarginBottom(double *aMarginBottom) override { return _to GetMarginBottom(aMarginBottom); } \
  NS_IMETHOD SetMarginBottom(double aMarginBottom) override { return _to SetMarginBottom(aMarginBottom); } \
  NS_IMETHOD GetMarginRight(double *aMarginRight) override { return _to GetMarginRight(aMarginRight); } \
  NS_IMETHOD SetMarginRight(double aMarginRight) override { return _to SetMarginRight(aMarginRight); } \
  NS_IMETHOD GetUnwriteableMarginTop(double *aUnwriteableMarginTop) override { return _to GetUnwriteableMarginTop(aUnwriteableMarginTop); } \
  NS_IMETHOD SetUnwriteableMarginTop(double aUnwriteableMarginTop) override { return _to SetUnwriteableMarginTop(aUnwriteableMarginTop); } \
  NS_IMETHOD GetUnwriteableMarginLeft(double *aUnwriteableMarginLeft) override { return _to GetUnwriteableMarginLeft(aUnwriteableMarginLeft); } \
  NS_IMETHOD SetUnwriteableMarginLeft(double aUnwriteableMarginLeft) override { return _to SetUnwriteableMarginLeft(aUnwriteableMarginLeft); } \
  NS_IMETHOD GetUnwriteableMarginBottom(double *aUnwriteableMarginBottom) override { return _to GetUnwriteableMarginBottom(aUnwriteableMarginBottom); } \
  NS_IMETHOD SetUnwriteableMarginBottom(double aUnwriteableMarginBottom) override { return _to SetUnwriteableMarginBottom(aUnwriteableMarginBottom); } \
  NS_IMETHOD GetUnwriteableMarginRight(double *aUnwriteableMarginRight) override { return _to GetUnwriteableMarginRight(aUnwriteableMarginRight); } \
  NS_IMETHOD SetUnwriteableMarginRight(double aUnwriteableMarginRight) override { return _to SetUnwriteableMarginRight(aUnwriteableMarginRight); } \
  NS_IMETHOD GetScaling(double *aScaling) override { return _to GetScaling(aScaling); } \
  NS_IMETHOD SetScaling(double aScaling) override { return _to SetScaling(aScaling); } \
  using nsIPrintSettings::GetPrintBGColors; \
  NS_IMETHOD GetPrintBGColors(bool *aPrintBGColors) override { return _to GetPrintBGColors(aPrintBGColors); } \
  NS_IMETHOD SetPrintBGColors(bool aPrintBGColors) override { return _to SetPrintBGColors(aPrintBGColors); } \
  using nsIPrintSettings::GetPrintBGImages; \
  NS_IMETHOD GetPrintBGImages(bool *aPrintBGImages) override { return _to GetPrintBGImages(aPrintBGImages); } \
  NS_IMETHOD SetPrintBGImages(bool aPrintBGImages) override { return _to SetPrintBGImages(aPrintBGImages); } \
  using nsIPrintSettings::GetHonorPageRuleMargins; \
  NS_IMETHOD GetHonorPageRuleMargins(bool *aHonorPageRuleMargins) override { return _to GetHonorPageRuleMargins(aHonorPageRuleMargins); } \
  NS_IMETHOD SetHonorPageRuleMargins(bool aHonorPageRuleMargins) override { return _to SetHonorPageRuleMargins(aHonorPageRuleMargins); } \
  using nsIPrintSettings::GetShowMarginGuides; \
  NS_IMETHOD GetShowMarginGuides(bool *aShowMarginGuides) override { return _to GetShowMarginGuides(aShowMarginGuides); } \
  NS_IMETHOD SetShowMarginGuides(bool aShowMarginGuides) override { return _to SetShowMarginGuides(aShowMarginGuides); } \
  using nsIPrintSettings::GetIsPrintSelectionRBEnabled; \
  NS_IMETHOD GetIsPrintSelectionRBEnabled(bool *aIsPrintSelectionRBEnabled) override { return _to GetIsPrintSelectionRBEnabled(aIsPrintSelectionRBEnabled); } \
  NS_IMETHOD SetIsPrintSelectionRBEnabled(bool aIsPrintSelectionRBEnabled) override { return _to SetIsPrintSelectionRBEnabled(aIsPrintSelectionRBEnabled); } \
  using nsIPrintSettings::GetPrintSelectionOnly; \
  NS_IMETHOD GetPrintSelectionOnly(bool *aPrintSelectionOnly) override { return _to GetPrintSelectionOnly(aPrintSelectionOnly); } \
  NS_IMETHOD SetPrintSelectionOnly(bool aPrintSelectionOnly) override { return _to SetPrintSelectionOnly(aPrintSelectionOnly); } \
  NS_IMETHOD GetTitle(nsAString& aTitle) override { return _to GetTitle(aTitle); } \
  NS_IMETHOD SetTitle(const nsAString& aTitle) override { return _to SetTitle(aTitle); } \
  NS_IMETHOD GetDocURL(nsAString& aDocURL) override { return _to GetDocURL(aDocURL); } \
  NS_IMETHOD SetDocURL(const nsAString& aDocURL) override { return _to SetDocURL(aDocURL); } \
  NS_IMETHOD GetHeaderStrLeft(nsAString& aHeaderStrLeft) override { return _to GetHeaderStrLeft(aHeaderStrLeft); } \
  NS_IMETHOD SetHeaderStrLeft(const nsAString& aHeaderStrLeft) override { return _to SetHeaderStrLeft(aHeaderStrLeft); } \
  NS_IMETHOD GetHeaderStrCenter(nsAString& aHeaderStrCenter) override { return _to GetHeaderStrCenter(aHeaderStrCenter); } \
  NS_IMETHOD SetHeaderStrCenter(const nsAString& aHeaderStrCenter) override { return _to SetHeaderStrCenter(aHeaderStrCenter); } \
  NS_IMETHOD GetHeaderStrRight(nsAString& aHeaderStrRight) override { return _to GetHeaderStrRight(aHeaderStrRight); } \
  NS_IMETHOD SetHeaderStrRight(const nsAString& aHeaderStrRight) override { return _to SetHeaderStrRight(aHeaderStrRight); } \
  NS_IMETHOD GetFooterStrLeft(nsAString& aFooterStrLeft) override { return _to GetFooterStrLeft(aFooterStrLeft); } \
  NS_IMETHOD SetFooterStrLeft(const nsAString& aFooterStrLeft) override { return _to SetFooterStrLeft(aFooterStrLeft); } \
  NS_IMETHOD GetFooterStrCenter(nsAString& aFooterStrCenter) override { return _to GetFooterStrCenter(aFooterStrCenter); } \
  NS_IMETHOD SetFooterStrCenter(const nsAString& aFooterStrCenter) override { return _to SetFooterStrCenter(aFooterStrCenter); } \
  NS_IMETHOD GetFooterStrRight(nsAString& aFooterStrRight) override { return _to GetFooterStrRight(aFooterStrRight); } \
  NS_IMETHOD SetFooterStrRight(const nsAString& aFooterStrRight) override { return _to SetFooterStrRight(aFooterStrRight); } \
  NS_IMETHOD GetIsCancelled(bool *aIsCancelled) override { return _to GetIsCancelled(aIsCancelled); } \
  NS_IMETHOD SetIsCancelled(bool aIsCancelled) override { return _to SetIsCancelled(aIsCancelled); } \
  NS_IMETHOD GetSaveOnCancel(bool *aSaveOnCancel) override { return _to GetSaveOnCancel(aSaveOnCancel); } \
  NS_IMETHOD GetPrintSilent(bool *aPrintSilent) override { return _to GetPrintSilent(aPrintSilent); } \
  NS_IMETHOD SetPrintSilent(bool aPrintSilent) override { return _to SetPrintSilent(aPrintSilent); } \
  NS_IMETHOD GetShrinkToFit(bool *aShrinkToFit) override { return _to GetShrinkToFit(aShrinkToFit); } \
  NS_IMETHOD SetShrinkToFit(bool aShrinkToFit) override { return _to SetShrinkToFit(aShrinkToFit); } \
  NS_IMETHOD GetShowPrintProgress(bool *aShowPrintProgress) override { return _to GetShowPrintProgress(aShowPrintProgress); } \
  NS_IMETHOD SetShowPrintProgress(bool aShowPrintProgress) override { return _to SetShowPrintProgress(aShowPrintProgress); } \
  NS_IMETHOD GetPaperId(nsAString& aPaperId) override { return _to GetPaperId(aPaperId); } \
  NS_IMETHOD SetPaperId(const nsAString& aPaperId) override { return _to SetPaperId(aPaperId); } \
  NS_IMETHOD GetPaperWidth(double *aPaperWidth) override { return _to GetPaperWidth(aPaperWidth); } \
  NS_IMETHOD SetPaperWidth(double aPaperWidth) override { return _to SetPaperWidth(aPaperWidth); } \
  NS_IMETHOD GetPaperHeight(double *aPaperHeight) override { return _to GetPaperHeight(aPaperHeight); } \
  NS_IMETHOD SetPaperHeight(double aPaperHeight) override { return _to SetPaperHeight(aPaperHeight); } \
  NS_IMETHOD GetPaperSizeUnit(int16_t *aPaperSizeUnit) override { return _to GetPaperSizeUnit(aPaperSizeUnit); } \
  NS_IMETHOD SetPaperSizeUnit(int16_t aPaperSizeUnit) override { return _to SetPaperSizeUnit(aPaperSizeUnit); } \
  NS_IMETHOD GetPrintReversed(bool *aPrintReversed) override { return _to GetPrintReversed(aPrintReversed); } \
  NS_IMETHOD SetPrintReversed(bool aPrintReversed) override { return _to SetPrintReversed(aPrintReversed); } \
  using nsIPrintSettings::GetPrintInColor; \
  NS_IMETHOD GetPrintInColor(bool *aPrintInColor) override { return _to GetPrintInColor(aPrintInColor); } \
  NS_IMETHOD SetPrintInColor(bool aPrintInColor) override { return _to SetPrintInColor(aPrintInColor); } \
  NS_IMETHOD GetOrientation(int32_t *aOrientation) override { return _to GetOrientation(aOrientation); } \
  NS_IMETHOD SetOrientation(int32_t aOrientation) override { return _to SetOrientation(aOrientation); } \
  NS_IMETHOD GetNumCopies(int32_t *aNumCopies) override { return _to GetNumCopies(aNumCopies); } \
  NS_IMETHOD SetNumCopies(int32_t aNumCopies) override { return _to SetNumCopies(aNumCopies); } \
  NS_IMETHOD GetNumPagesPerSheet(int32_t *aNumPagesPerSheet) override { return _to GetNumPagesPerSheet(aNumPagesPerSheet); } \
  NS_IMETHOD SetNumPagesPerSheet(int32_t aNumPagesPerSheet) override { return _to SetNumPagesPerSheet(aNumPagesPerSheet); } \
  NS_IMETHOD GetPrinterName(nsAString& aPrinterName) override { return _to GetPrinterName(aPrinterName); } \
  NS_IMETHOD SetPrinterName(const nsAString& aPrinterName) override { return _to SetPrinterName(aPrinterName); } \
  NS_IMETHOD GetPrintToFile(bool *aPrintToFile) override { return _to GetPrintToFile(aPrintToFile); } \
  NS_IMETHOD SetPrintToFile(bool aPrintToFile) override { return _to SetPrintToFile(aPrintToFile); } \
  NS_IMETHOD GetToFileName(nsAString& aToFileName) override { return _to GetToFileName(aToFileName); } \
  NS_IMETHOD SetToFileName(const nsAString& aToFileName) override { return _to SetToFileName(aToFileName); } \
  NS_IMETHOD GetOutputFormat(int16_t *aOutputFormat) override { return _to GetOutputFormat(aOutputFormat); } \
  NS_IMETHOD SetOutputFormat(int16_t aOutputFormat) override { return _to SetOutputFormat(aOutputFormat); } \
  NS_IMETHOD GetPrintPageDelay(int32_t *aPrintPageDelay) override { return _to GetPrintPageDelay(aPrintPageDelay); } \
  NS_IMETHOD SetPrintPageDelay(int32_t aPrintPageDelay) override { return _to SetPrintPageDelay(aPrintPageDelay); } \
  NS_IMETHOD GetResolution(int32_t *aResolution) override { return _to GetResolution(aResolution); } \
  NS_IMETHOD SetResolution(int32_t aResolution) override { return _to SetResolution(aResolution); } \
  NS_IMETHOD GetDuplex(int32_t *aDuplex) override { return _to GetDuplex(aDuplex); } \
  NS_IMETHOD SetDuplex(int32_t aDuplex) override { return _to SetDuplex(aDuplex); } \
  NS_IMETHOD GetIsInitializedFromPrinter(bool *aIsInitializedFromPrinter) override { return _to GetIsInitializedFromPrinter(aIsInitializedFromPrinter); } \
  NS_IMETHOD SetIsInitializedFromPrinter(bool aIsInitializedFromPrinter) override { return _to SetIsInitializedFromPrinter(aIsInitializedFromPrinter); } \
  NS_IMETHOD GetIsInitializedFromPrefs(bool *aIsInitializedFromPrefs) override { return _to GetIsInitializedFromPrefs(aIsInitializedFromPrefs); } \
  NS_IMETHOD SetIsInitializedFromPrefs(bool aIsInitializedFromPrefs) override { return _to SetIsInitializedFromPrefs(aIsInitializedFromPrefs); } \
  NS_IMETHOD SetMarginInTwips(nsIntMargin & aMargin) override { return _to SetMarginInTwips(aMargin); } \
  NS_IMETHOD SetEdgeInTwips(nsIntMargin & aEdge) override { return _to SetEdgeInTwips(aEdge); } \
  virtual nsIntMargin GetMarginInTwips(void) override { return _to GetMarginInTwips(); } \
  virtual nsIntMargin GetEdgeInTwips(void) override { return _to GetEdgeInTwips(); } \
  NS_IMETHOD SetupSilentPrinting(void) override { return _to SetupSilentPrinting(); } \
  NS_IMETHOD SetUnwriteableMarginInTwips(nsIntMargin & aEdge) override { return _to SetUnwriteableMarginInTwips(aEdge); } \
  virtual nsIntMargin GetUnwriteableMarginInTwips(void) override { return _to GetUnwriteableMarginInTwips(); } \
  NS_IMETHOD GetPageRanges(nsTArray<int32_t >& aPageRanges) override { return _to GetPageRanges(aPageRanges); } \
  NS_IMETHOD SetPageRanges(const nsTArray<int32_t >& aPageRanges) override { return _to SetPageRanges(aPageRanges); } \

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPRINTSETTINGS(_to) \
  NS_IMETHOD GetEffectivePageSize(double *aWidth, double *aHeight) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetEffectivePageSize(aWidth, aHeight); } \
  virtual void GetEffectiveSheetSize(double *aWidth, double *aHeight) override; \
  virtual int32_t GetSheetOrientation(void) override; \
  virtual bool HasOrthogonalSheetsAndPages(void) override; \
  NS_IMETHOD Clone(nsIPrintSettings **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Clone(_retval); } \
  NS_IMETHOD Assign(nsIPrintSettings *aPS) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Assign(aPS); } \
  NS_IMETHOD GetPrintSession(nsIPrintSession **aPrintSession) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPrintSession(aPrintSession); } \
  NS_IMETHOD SetPrintSession(nsIPrintSession *aPrintSession) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetPrintSession(aPrintSession); } \
  NS_IMETHOD GetEdgeTop(double *aEdgeTop) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetEdgeTop(aEdgeTop); } \
  NS_IMETHOD SetEdgeTop(double aEdgeTop) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetEdgeTop(aEdgeTop); } \
  NS_IMETHOD GetEdgeLeft(double *aEdgeLeft) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetEdgeLeft(aEdgeLeft); } \
  NS_IMETHOD SetEdgeLeft(double aEdgeLeft) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetEdgeLeft(aEdgeLeft); } \
  NS_IMETHOD GetEdgeBottom(double *aEdgeBottom) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetEdgeBottom(aEdgeBottom); } \
  NS_IMETHOD SetEdgeBottom(double aEdgeBottom) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetEdgeBottom(aEdgeBottom); } \
  NS_IMETHOD GetEdgeRight(double *aEdgeRight) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetEdgeRight(aEdgeRight); } \
  NS_IMETHOD SetEdgeRight(double aEdgeRight) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetEdgeRight(aEdgeRight); } \
  NS_IMETHOD GetMarginTop(double *aMarginTop) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMarginTop(aMarginTop); } \
  NS_IMETHOD SetMarginTop(double aMarginTop) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetMarginTop(aMarginTop); } \
  NS_IMETHOD GetMarginLeft(double *aMarginLeft) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMarginLeft(aMarginLeft); } \
  NS_IMETHOD SetMarginLeft(double aMarginLeft) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetMarginLeft(aMarginLeft); } \
  NS_IMETHOD GetMarginBottom(double *aMarginBottom) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMarginBottom(aMarginBottom); } \
  NS_IMETHOD SetMarginBottom(double aMarginBottom) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetMarginBottom(aMarginBottom); } \
  NS_IMETHOD GetMarginRight(double *aMarginRight) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMarginRight(aMarginRight); } \
  NS_IMETHOD SetMarginRight(double aMarginRight) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetMarginRight(aMarginRight); } \
  NS_IMETHOD GetUnwriteableMarginTop(double *aUnwriteableMarginTop) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetUnwriteableMarginTop(aUnwriteableMarginTop); } \
  NS_IMETHOD SetUnwriteableMarginTop(double aUnwriteableMarginTop) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetUnwriteableMarginTop(aUnwriteableMarginTop); } \
  NS_IMETHOD GetUnwriteableMarginLeft(double *aUnwriteableMarginLeft) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetUnwriteableMarginLeft(aUnwriteableMarginLeft); } \
  NS_IMETHOD SetUnwriteableMarginLeft(double aUnwriteableMarginLeft) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetUnwriteableMarginLeft(aUnwriteableMarginLeft); } \
  NS_IMETHOD GetUnwriteableMarginBottom(double *aUnwriteableMarginBottom) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetUnwriteableMarginBottom(aUnwriteableMarginBottom); } \
  NS_IMETHOD SetUnwriteableMarginBottom(double aUnwriteableMarginBottom) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetUnwriteableMarginBottom(aUnwriteableMarginBottom); } \
  NS_IMETHOD GetUnwriteableMarginRight(double *aUnwriteableMarginRight) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetUnwriteableMarginRight(aUnwriteableMarginRight); } \
  NS_IMETHOD SetUnwriteableMarginRight(double aUnwriteableMarginRight) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetUnwriteableMarginRight(aUnwriteableMarginRight); } \
  NS_IMETHOD GetScaling(double *aScaling) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetScaling(aScaling); } \
  NS_IMETHOD SetScaling(double aScaling) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetScaling(aScaling); } \
  NS_IMETHOD GetPrintBGColors(bool *aPrintBGColors) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPrintBGColors(aPrintBGColors); } \
  NS_IMETHOD SetPrintBGColors(bool aPrintBGColors) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetPrintBGColors(aPrintBGColors); } \
  NS_IMETHOD GetPrintBGImages(bool *aPrintBGImages) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPrintBGImages(aPrintBGImages); } \
  NS_IMETHOD SetPrintBGImages(bool aPrintBGImages) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetPrintBGImages(aPrintBGImages); } \
  NS_IMETHOD GetHonorPageRuleMargins(bool *aHonorPageRuleMargins) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHonorPageRuleMargins(aHonorPageRuleMargins); } \
  NS_IMETHOD SetHonorPageRuleMargins(bool aHonorPageRuleMargins) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetHonorPageRuleMargins(aHonorPageRuleMargins); } \
  NS_IMETHOD GetShowMarginGuides(bool *aShowMarginGuides) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetShowMarginGuides(aShowMarginGuides); } \
  NS_IMETHOD SetShowMarginGuides(bool aShowMarginGuides) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetShowMarginGuides(aShowMarginGuides); } \
  NS_IMETHOD GetIsPrintSelectionRBEnabled(bool *aIsPrintSelectionRBEnabled) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsPrintSelectionRBEnabled(aIsPrintSelectionRBEnabled); } \
  NS_IMETHOD SetIsPrintSelectionRBEnabled(bool aIsPrintSelectionRBEnabled) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetIsPrintSelectionRBEnabled(aIsPrintSelectionRBEnabled); } \
  NS_IMETHOD GetPrintSelectionOnly(bool *aPrintSelectionOnly) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPrintSelectionOnly(aPrintSelectionOnly); } \
  NS_IMETHOD SetPrintSelectionOnly(bool aPrintSelectionOnly) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetPrintSelectionOnly(aPrintSelectionOnly); } \
  NS_IMETHOD GetTitle(nsAString& aTitle) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTitle(aTitle); } \
  NS_IMETHOD SetTitle(const nsAString& aTitle) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetTitle(aTitle); } \
  NS_IMETHOD GetDocURL(nsAString& aDocURL) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDocURL(aDocURL); } \
  NS_IMETHOD SetDocURL(const nsAString& aDocURL) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetDocURL(aDocURL); } \
  NS_IMETHOD GetHeaderStrLeft(nsAString& aHeaderStrLeft) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHeaderStrLeft(aHeaderStrLeft); } \
  NS_IMETHOD SetHeaderStrLeft(const nsAString& aHeaderStrLeft) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetHeaderStrLeft(aHeaderStrLeft); } \
  NS_IMETHOD GetHeaderStrCenter(nsAString& aHeaderStrCenter) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHeaderStrCenter(aHeaderStrCenter); } \
  NS_IMETHOD SetHeaderStrCenter(const nsAString& aHeaderStrCenter) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetHeaderStrCenter(aHeaderStrCenter); } \
  NS_IMETHOD GetHeaderStrRight(nsAString& aHeaderStrRight) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHeaderStrRight(aHeaderStrRight); } \
  NS_IMETHOD SetHeaderStrRight(const nsAString& aHeaderStrRight) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetHeaderStrRight(aHeaderStrRight); } \
  NS_IMETHOD GetFooterStrLeft(nsAString& aFooterStrLeft) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFooterStrLeft(aFooterStrLeft); } \
  NS_IMETHOD SetFooterStrLeft(const nsAString& aFooterStrLeft) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetFooterStrLeft(aFooterStrLeft); } \
  NS_IMETHOD GetFooterStrCenter(nsAString& aFooterStrCenter) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFooterStrCenter(aFooterStrCenter); } \
  NS_IMETHOD SetFooterStrCenter(const nsAString& aFooterStrCenter) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetFooterStrCenter(aFooterStrCenter); } \
  NS_IMETHOD GetFooterStrRight(nsAString& aFooterStrRight) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFooterStrRight(aFooterStrRight); } \
  NS_IMETHOD SetFooterStrRight(const nsAString& aFooterStrRight) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetFooterStrRight(aFooterStrRight); } \
  NS_IMETHOD GetIsCancelled(bool *aIsCancelled) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsCancelled(aIsCancelled); } \
  NS_IMETHOD SetIsCancelled(bool aIsCancelled) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetIsCancelled(aIsCancelled); } \
  NS_IMETHOD GetSaveOnCancel(bool *aSaveOnCancel) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSaveOnCancel(aSaveOnCancel); } \
  NS_IMETHOD GetPrintSilent(bool *aPrintSilent) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPrintSilent(aPrintSilent); } \
  NS_IMETHOD SetPrintSilent(bool aPrintSilent) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetPrintSilent(aPrintSilent); } \
  NS_IMETHOD GetShrinkToFit(bool *aShrinkToFit) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetShrinkToFit(aShrinkToFit); } \
  NS_IMETHOD SetShrinkToFit(bool aShrinkToFit) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetShrinkToFit(aShrinkToFit); } \
  NS_IMETHOD GetShowPrintProgress(bool *aShowPrintProgress) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetShowPrintProgress(aShowPrintProgress); } \
  NS_IMETHOD SetShowPrintProgress(bool aShowPrintProgress) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetShowPrintProgress(aShowPrintProgress); } \
  NS_IMETHOD GetPaperId(nsAString& aPaperId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPaperId(aPaperId); } \
  NS_IMETHOD SetPaperId(const nsAString& aPaperId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetPaperId(aPaperId); } \
  NS_IMETHOD GetPaperWidth(double *aPaperWidth) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPaperWidth(aPaperWidth); } \
  NS_IMETHOD SetPaperWidth(double aPaperWidth) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetPaperWidth(aPaperWidth); } \
  NS_IMETHOD GetPaperHeight(double *aPaperHeight) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPaperHeight(aPaperHeight); } \
  NS_IMETHOD SetPaperHeight(double aPaperHeight) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetPaperHeight(aPaperHeight); } \
  NS_IMETHOD GetPaperSizeUnit(int16_t *aPaperSizeUnit) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPaperSizeUnit(aPaperSizeUnit); } \
  NS_IMETHOD SetPaperSizeUnit(int16_t aPaperSizeUnit) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetPaperSizeUnit(aPaperSizeUnit); } \
  NS_IMETHOD GetPrintReversed(bool *aPrintReversed) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPrintReversed(aPrintReversed); } \
  NS_IMETHOD SetPrintReversed(bool aPrintReversed) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetPrintReversed(aPrintReversed); } \
  NS_IMETHOD GetPrintInColor(bool *aPrintInColor) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPrintInColor(aPrintInColor); } \
  NS_IMETHOD SetPrintInColor(bool aPrintInColor) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetPrintInColor(aPrintInColor); } \
  NS_IMETHOD GetOrientation(int32_t *aOrientation) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetOrientation(aOrientation); } \
  NS_IMETHOD SetOrientation(int32_t aOrientation) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetOrientation(aOrientation); } \
  NS_IMETHOD GetNumCopies(int32_t *aNumCopies) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNumCopies(aNumCopies); } \
  NS_IMETHOD SetNumCopies(int32_t aNumCopies) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetNumCopies(aNumCopies); } \
  NS_IMETHOD GetNumPagesPerSheet(int32_t *aNumPagesPerSheet) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNumPagesPerSheet(aNumPagesPerSheet); } \
  NS_IMETHOD SetNumPagesPerSheet(int32_t aNumPagesPerSheet) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetNumPagesPerSheet(aNumPagesPerSheet); } \
  NS_IMETHOD GetPrinterName(nsAString& aPrinterName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPrinterName(aPrinterName); } \
  NS_IMETHOD SetPrinterName(const nsAString& aPrinterName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetPrinterName(aPrinterName); } \
  NS_IMETHOD GetPrintToFile(bool *aPrintToFile) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPrintToFile(aPrintToFile); } \
  NS_IMETHOD SetPrintToFile(bool aPrintToFile) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetPrintToFile(aPrintToFile); } \
  NS_IMETHOD GetToFileName(nsAString& aToFileName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetToFileName(aToFileName); } \
  NS_IMETHOD SetToFileName(const nsAString& aToFileName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetToFileName(aToFileName); } \
  NS_IMETHOD GetOutputFormat(int16_t *aOutputFormat) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetOutputFormat(aOutputFormat); } \
  NS_IMETHOD SetOutputFormat(int16_t aOutputFormat) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetOutputFormat(aOutputFormat); } \
  NS_IMETHOD GetPrintPageDelay(int32_t *aPrintPageDelay) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPrintPageDelay(aPrintPageDelay); } \
  NS_IMETHOD SetPrintPageDelay(int32_t aPrintPageDelay) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetPrintPageDelay(aPrintPageDelay); } \
  NS_IMETHOD GetResolution(int32_t *aResolution) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetResolution(aResolution); } \
  NS_IMETHOD SetResolution(int32_t aResolution) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetResolution(aResolution); } \
  NS_IMETHOD GetDuplex(int32_t *aDuplex) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDuplex(aDuplex); } \
  NS_IMETHOD SetDuplex(int32_t aDuplex) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetDuplex(aDuplex); } \
  NS_IMETHOD GetIsInitializedFromPrinter(bool *aIsInitializedFromPrinter) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsInitializedFromPrinter(aIsInitializedFromPrinter); } \
  NS_IMETHOD SetIsInitializedFromPrinter(bool aIsInitializedFromPrinter) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetIsInitializedFromPrinter(aIsInitializedFromPrinter); } \
  NS_IMETHOD GetIsInitializedFromPrefs(bool *aIsInitializedFromPrefs) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsInitializedFromPrefs(aIsInitializedFromPrefs); } \
  NS_IMETHOD SetIsInitializedFromPrefs(bool aIsInitializedFromPrefs) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetIsInitializedFromPrefs(aIsInitializedFromPrefs); } \
  NS_IMETHOD SetMarginInTwips(nsIntMargin & aMargin) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetMarginInTwips(aMargin); } \
  NS_IMETHOD SetEdgeInTwips(nsIntMargin & aEdge) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetEdgeInTwips(aEdge); } \
  virtual nsIntMargin GetMarginInTwips(void) override; \
  virtual nsIntMargin GetEdgeInTwips(void) override; \
  NS_IMETHOD SetupSilentPrinting(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetupSilentPrinting(); } \
  NS_IMETHOD SetUnwriteableMarginInTwips(nsIntMargin & aEdge) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetUnwriteableMarginInTwips(aEdge); } \
  virtual nsIntMargin GetUnwriteableMarginInTwips(void) override; \
  NS_IMETHOD GetPageRanges(nsTArray<int32_t >& aPageRanges) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPageRanges(aPageRanges); } \
  NS_IMETHOD SetPageRanges(const nsTArray<int32_t >& aPageRanges) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetPageRanges(aPageRanges); } \

namespace mozilla {
struct PrintSettingsInitializer;
}
already_AddRefed<nsIPrintSettings> CreatePlatformPrintSettings(const mozilla::PrintSettingsInitializer&);

#endif /* __gen_nsIPrintSettings_h__ */
