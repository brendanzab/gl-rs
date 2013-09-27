Registry generator that uses the [New XML API Registry]
(http://www.opengl.org/discussion_boards/showthread.php/181927-New-XML-based-API-Registry-released?p=1251775).

## Instructions

1. Compile `generator.rs` using `rustc`. You will need [sax-rs]
   (https://github.com/bjz/sax-rs) and [libxml2](http://www.xmlsoft.org/).

2. Download the XML registry from the [Kronos public SVN repository]
   (https://cvs.khronos.org/svn/repos/ogl/trunk/doc/registry/public/api/)

~~~
wget --no-check-certificate https://cvs.khronos.org/svn/repos/ogl/trunk/doc/registry/public/api/gl.xml
~~~

3. Generate the loader

~~~
./generator > gl.rs
~~~

You can pick which GL version to use, as well as the profile, and any
extensions. See `./generator --help` for the options to pass. The default is
4.3 core profile with no extensions.

Some other examples:

~~~
./generator --version 3.3 --profile core
~~~

~~~
./generator --version 2.1 --extension GL_ARB_robustness --extension GL_EXT_draw_instanced
~~~

You can also use `--namespace` for GLX and WGL, but unfortunately their
loaders are not finished at this time. Help to remedy this is welcomed!

## Struct style loader

I'm not sure how useful this is, but I've also included the option to generate
a loader that stores the pointers in a struct. This might be useful for people
who want to set up multiple contexts in the same application.

~~~
./generator --type struct > gl_struct.rs
~~~
