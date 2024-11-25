#!/system/bin/sh
# Copyright 2023 shadow3aaa@gitbub.com
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

MODDIR=${0%/*}
DIR=/sdcard/Android/fas-rs
LOG=$MODDIR/log.txt

killall -15 touch_sampling
chmod +x ${0%/*}/touch_sampling
RUST_BACKTRACE=1 nohup $MODDIR/touch_sampling $DIR/games.toml 240 >$LOG 2>&1 &
