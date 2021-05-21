/* This file is auto-generated, only for internal use in
   TelemetryUserInteraction.h, see gen_userinteraction_data.py. */

#ifndef mozilla_TelemetryUserInteractionData_h
#define mozilla_TelemetryUserInteractionData_h
#include "core/UserInteractionInfo.h"


      namespace mozilla {
      namespace Telemetry {
      namespace UserInteractionID {
        const static uint32_t UserInteractionCount = 2;
      }  // namespace UserInteractionID
      }  // namespace Telemetry
      }  // namespace mozilla
    
namespace {
constexpr UserInteractionInfo gUserInteractions[] = {
  UserInteractionInfo(0),
  UserInteractionInfo(21),
};
static_assert(sizeof(gUserInteractions) <= UINT32_MAX, "index overflow of UserInteractionInfo table gUserInteractions");
}  // namespace

#if defined(_MSC_VER) && !defined(__clang__)
const char gUserInteractionsStringTable[] = {
#else
constexpr char gUserInteractionsStringTable[] = {
#endif
  /*     0 - "browser.tabs.opening" */ 'b', 'r', 'o', 'w', 's', 'e', 'r', '.', 't', 'a', 'b', 's', '.', 'o', 'p', 'e', 'n', 'i', 'n', 'g', '\0',
  /*    21 - "testing.interaction" */ 't', 'e', 's', 't', 'i', 'n', 'g', '.', 'i', 'n', 't', 'e', 'r', 'a', 'c', 't', 'i', 'o', 'n', '\0',
};

static_assert(sizeof(gUserInteractionsStringTable) <= UINT32_MAX, "index overflow");

#endif // mozilla_TelemetryUserInteractionData_h

