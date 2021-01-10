# 一个音视频处理工具

- ### 使用到的技术
    - ##### [rust](https://www.rust-lang.org)
    - ##### [gstreamer](https://gstreamer.freedesktop.org)
    - ##### [ffmpeg](https://ffmpeg.org)

- ### 目录结构
    + ave-video：视频处理
    + ave-image：图片处理
    + ave-tool：常用工具
    + iced-style：自定义iced样式库

- ### 分支介绍
    + [master](https://github.com/schizobulia/ave)：只存在说明文件
    + [ffmpeg](https://github.com/schizobulia/ave/tree/ffmpeg)：使用ffmpeg开发(推荐使用)
    + [gstreamer](https://github.com/schizobulia/ave/tree/gstreamer)：使用gstreamer开发,但更多的是学习音视频处理(不推荐使用)

- ### 打包好的exe
    + 视频处理(推荐)：http://demo.51jcjgzy.cn/ave/ave-video-ffmpeg-dev.zip
        - [x] 格式转换
        - [x] 压缩控制

    + 视频处理：http://demo.51jcjgzy.cn/ave/ave-video-dev.zip
        - [x] 格式转换

    + 图片处理：http://demo.51jcjgzy.cn/ave/ave-image-dev.exe
        - [x] 格式转换
        - [x] 压缩控制

- ### 开发进度
    + 如果有兴趣参与,可以先查看[开发进度](https://github.com/schizobulia/ave/projects/1)。
    + 添加新功能,请先Pr到对应的分支
