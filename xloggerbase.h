// Tencent is pleased to support the open source community by making Mars available.
// Copyright (C) 2016 THL A29 Limited, a Tencent company. All rights reserved.

// Licensed under the MIT License (the "License"); you may not use this file except in 
// compliance with the License. You may obtain a copy of the License at
// http://opensource.org/licenses/MIT

// Unless required by applicable law or agreed to in writing, software distributed under the License is
// distributed on an "AS IS" basis, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND,
// either express or implied. See the License for the specific language governing permissions and
// limitations under the License.


/*
 ============================================================================
 ============================================================================
 */

#ifndef XLOGGERBASE_H_
#define XLOGGERBASE_H_

#include <sys/time.h>
#include <time.h>
#include <stdarg.h>
#include <stdint.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

enum TAppenderMode
{
    kAppednerAsync,
    kAppednerSync,
};

typedef enum {
    kLevelAll = 0,
    kLevelVerbose = 0,
    kLevelDebug,    // Detailed information on the flow through the system.
    kLevelInfo,     // Interesting runtime events (startup/shutdown), should be conservative and keep to a minimum.
    kLevelWarn,     // Other runtime situations that are undesirable or unexpected, but not necessarily "wrong".
    kLevelError,    // Other runtime errors or unexpected conditions.
    kLevelFatal,    // Severe errors that cause premature termination.
    kLevelNone,     // Special level used to disable all log messages.
} TLogLevel;

typedef struct XLoggerInfo_t {
    TLogLevel level;
    const char* tag;
    const char* filename;
    const char* func_name;
    int line;

    struct timeval timeval;
    intmax_t pid;
    intmax_t tid;
    intmax_t maintid;
} XLoggerInfo;

void xlogger_Write(const XLoggerInfo* _info, const char* _log);

void appender_open(enum TAppenderMode _mode, const char* _dir, const char* _nameprefix, const char* _pub_key);
void appender_close();
void appender_set_console_log(bool _is_open);

#ifdef __cplusplus
}
#endif

#endif
