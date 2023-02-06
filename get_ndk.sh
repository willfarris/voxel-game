if [ ! -d "NDK/android-ndk-r22b" ]; then
    echo "Missing  Android NDK in 'NDK/android-ndk-r22b', installing..."
    mkdir -p NDK
    cd NDK
    wget https://dl.google.com/android/repository/android-ndk-r22b-$(uname -s)-x86_64.zip && \
    unzip android-ndk-r22b-$(uname -s)-x86_64.zip && \
    rm android-ndk-r22b-$(uname -s)-x86_64.zip && \
    cd ..
fi
