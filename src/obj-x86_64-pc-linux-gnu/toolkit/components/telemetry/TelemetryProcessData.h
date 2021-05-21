/* This file is auto-generated from Telemetry build scripts,
   see gen_process_data.py. */

#ifndef mozilla_TelemetryProcessData_h
#define mozilla_TelemetryProcessData_h

#include "mozilla/TelemetryProcessEnums.h"

namespace mozilla {
namespace Telemetry {

static GeckoProcessType ProcessIDToGeckoProcessType[6] = {
  /* 0: ProcessID::Content = */ GeckoProcessType_Content,
  /* 1: ProcessID::Dynamic = */ GeckoProcessType_Default,
  /* 2: ProcessID::Extension = */ GeckoProcessType_Content,
  /* 3: ProcessID::Gpu = */ GeckoProcessType_GPU,
  /* 4: ProcessID::Parent = */ GeckoProcessType_Default,
  /* 5: ProcessID::Socket = */ GeckoProcessType_Socket,
};

#if defined(_MSC_VER) && !defined(__clang__)
static const char* const ProcessIDToString[6] = {
#else
static constexpr const char* ProcessIDToString[6] = {
#endif
  /* 0: ProcessID::Content = */ "content",
  /* 1: ProcessID::Dynamic = */ "dynamic",
  /* 2: ProcessID::Extension = */ "extension",
  /* 3: ProcessID::Gpu = */ "gpu",
  /* 4: ProcessID::Parent = */ "parent",
  /* 5: ProcessID::Socket = */ "socket",
};

} // namespace Telemetry
} // namespace mozilla
#endif // mozilla_TelemetryProcessData_h
