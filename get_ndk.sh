#NDK_VERSION="android-ndk-r22b"
NDK_VERSION="android-ndk-r25c"

if [ ! -d "NDK/$NDK_VERSION" ]; then
    echo "Missing  Android NDK in 'NDK/$NDK_VERSION', installing..."
    mkdir -p NDK
    cd NDK
    wget https://dl.google.com/android/repository/$NDK_VERSION-$(uname -s).zip && \
    unzip $NDK_VERSION-$(uname -s).zip && \
    rm $NDK_VERSION-$(uname -s).zip && \
    cd ..
fi
