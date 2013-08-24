Registry generator that uses the [New XML API Registry](http://www.opengl.org/discussion_boards/showthread.php/181927-New-XML-based-API-Registry-released?p=1251775).

## Instructions

1. Compile `generator.rs` using `rustc`. You will need [sax-rs](https://github.com/bjz/sax-rs) and [libxml2](http://www.xmlsoft.org/).

2. Download the XML registry from the [Kronos public SVN repository](https://cvs.khronos.org/svn/repos/ogl/trunk/doc/registry/public/api/)

~~~
wget --no-check-certificate https://cvs.khronos.org/svn/repos/ogl/trunk/doc/registry/public/api/gl.xml
~~~

3. Generate the loaders

~~~
./generator gl ptr > gl_ptr.rs
./generator gl struct > gl_struct.rs
~~~

Unfortunately the GLX and WGL loaders are not finished at this time. Help to
remedy this is welcomed!
