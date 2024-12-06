      Quickstart - setuptools 75.6.0.post20241124 documentation     body { --color-code-background: #f8f8f8; --color-code-foreground: black; --color-brand-primary: #336790; --color-brand-content: #336790; } @media not print { body\[data-theme="dark"\] { --color-code-background: #202020; --color-code-foreground: #d0d0d0; --color-brand-primary: #E5B62F; --color-brand-content: #E5B62F; } @media (prefers-color-scheme: dark) { body:not(\[data-theme="light"\]) { --color-code-background: #202020; --color-code-foreground: #d0d0d0; --color-brand-primary: #E5B62F; --color-brand-content: #E5B62F; } } }  document.body.dataset.theme = localStorage.getItem("theme") || "auto"; Contents Menu Expand Light mode Dark mode Auto light/dark, in light mode Auto light/dark, in dark mode  

Hide navigation sidebar

Hide table of contents sidebar

[Skip to content](#furo-main-content)

Toggle site navigation sidebar

[

setuptools 75.6.0.post20241124 documentation

](../index.html)

Toggle Light / Dark / Auto color theme

Toggle table of contents sidebar

[

![Logo](../_static/logo.svg)

](../index.html)

  

Links

*   [Home](https://github.com/pypa/setuptools)
*   [PyPI](https://pypi.org/project/setuptools)

*   [User guide](index.html)
    
    Toggle navigation of User guide
    
    *   [Quickstart](#)
    *   [Package Discovery and Namespace Packages](package_discovery.html)
    *   [Dependencies Management in Setuptools](dependency_management.html)
    *   [Development Mode (a.k.a. “Editable Installs”)](development_mode.html)
    *   [Entry Points](entry_point.html)
    *   [Data Files Support](datafiles.html)
    *   [Building Extension Modules](ext_modules.html)
    *   [Specifying Your Project’s Version](distribution.html)
    *   [Controlling files in the distribution](miscellaneous.html)
    *   [Caching and Troubleshooting](miscellaneous.html#caching-and-troubleshooting)
    *   [Extending or Customizing Setuptools](extension.html)
    *   [Configuring setuptools using `setup.cfg` files](declarative_config.html)
    *   [Configuring setuptools using `pyproject.toml` files](pyproject_config.html)
*   [Build System Support](../build_meta.html)
*   [Package Discovery and Resource Access using `pkg_resources`](../pkg_resources.html)
*   [Keywords](../references/keywords.html)
*   [Building and Distributing Packages with Setuptools](../setuptools.html)

Project

*   [Roadmap](../roadmap.html)
*   [Development guide](../development/index.html)
    
    Toggle navigation of Development guide
    
    *   [Developer’s Guide for Setuptools](../development/developer-guide.html)
    *   [Release Process](../development/releases.html)
*   [Backward compatibility & deprecated practice](../deprecated/index.html)
    
    Toggle navigation of Backward compatibility & deprecated practice
    
    *   [New and Changed `setup()` Keywords](../deprecated/changed_keywords.html)
    *   [Specifying dependencies that aren’t in PyPI via `dependency_links`](../deprecated/dependency_links.html)
    *   [The Internal Structure of Python Eggs](../deprecated/python_eggs.html)
    *   [Easy Install](../deprecated/easy_install.html)
    *   [Understanding the `zip_safe` flag](../deprecated/zip_safe.html)
    *   [Automatic Resource Extraction](../deprecated/resource_extraction.html)
    *   [Distributing Python Modules (Legacy version)](../deprecated/distutils/index.html)
        
        Toggle navigation of Distributing Python Modules (Legacy version)
        
        *   [1\. An Introduction to Distutils](../deprecated/distutils/introduction.html)
        *   [2\. Writing the Setup Script](../deprecated/distutils/setupscript.html)
        *   [3\. Writing the Setup Configuration File](../deprecated/distutils/configfile.html)
        *   [4\. Creating a Source Distribution](../deprecated/distutils/sourcedist.html)
        *   [5\. Creating Built Distributions](../deprecated/distutils/builtdist.html)
        *   [6\. Distutils Examples](../deprecated/distutils/examples.html)
        *   [7\. Extending Distutils](../deprecated/distutils/extending.html)
        *   [8\. Command Reference](../deprecated/distutils/commandref.html)
        *   [9\. API Reference](../deprecated/distutils/apiref.html)
    *   [Porting from Distutils](../deprecated/distutils-legacy.html)
    *   [“Eggsecutable” Scripts](../deprecated/functionalities.html)
    *   [Running `setuptools` commands](../deprecated/commands.html)
*   [History](../history.html)
*   [Credits](../history.html#credits)
*   [Artwork](../artwork.html)

[Back to top](#)

[View this page](../_sources/userguide/quickstart.rst.txt "View this page")

Toggle Light / Dark / Auto color theme

Toggle table of contents sidebar

Quickstart[¶](#quickstart "Link to this heading")
=================================================

Installation[¶](#installation "Link to this heading")
-----------------------------------------------------

You can install the latest version of `setuptools` using [pip](https://pypi.org/project/pip):

pip install \--upgrade setuptools\[core\]

Most of the times, however, you don’t have to…

Instead, when creating new Python packages, it is recommended to use a command line tool called [build](https://pypi.org/project/build). This tool will automatically download `setuptools` and any other build-time dependencies that your project might have. You just need to specify them in a `pyproject.toml` file at the root of your package, as indicated in the [following section](#basic-use).

You can also [install build](https://build.pypa.io/en/latest/installation.html "(in build v1.2.2.post1)") using [pip](https://pypi.org/project/pip):

pip install \--upgrade build

This will allow you to run the command: `python -m build`.

Important

Please note that some operating systems might be equipped with the `python3` and `pip3` commands instead of `python` and `pip` (but they should be equivalent). If you don’t have `pip` or `pip3` available in your system, please check out [pip installation docs](https://pip.pypa.io/en/latest/installation/ "(in pip v25.0)").

Every python package must provide a `pyproject.toml` and specify the backend (build system) it wants to use. The distribution can then be generated with whatever tool that provides a `build sdist`\-like functionality.

Basic Use[¶](#basic-use "Link to this heading")
-----------------------------------------------

When creating a Python package, you must provide a `pyproject.toml` file containing a `build-system` section similar to the example below:

\[build-system\]
requires \= \["setuptools"\]
build-backend \= "setuptools.build\_meta"

This section declares what are your build system dependencies, and which library will be used to actually do the packaging.

Note

Package maintainers might be tempted to use `setuptools[core]` as the requirement, given the guidance above. Avoid doing so, as the extra is currently considered an internal implementation detail and is likely to go away in the future and the Setuptools team will not support compatibility for problems arising from packages published with this extra declared. Vendored packages will satisfy the dependencies in the most common isolated build scenarios.

Note

Historically this documentation has unnecessarily listed `wheel` in the `requires` list, and many projects still do that. This is not recommended, as the backend no longer requires the `wheel` package, and listing it explicitly causes it to be unnecessarily required for source distribution builds. You should only include `wheel` in `requires` if you need to explicitly access it during build time (e.g. if your project needs a `setup.py` script that imports `wheel`).

In addition to specifying a build system, you also will need to add some package information such as metadata, contents, dependencies, etc. This can be done in the same `pyproject.toml` file, or in a separated one: `setup.cfg` or `setup.py` [\[1\]](#setup-py).

The following example demonstrates a minimum configuration (which assumes the project depends on [requests](https://pypi.org/project/requests) and [importlib-metadata](https://pypi.org/project/importlib-metadata) to be able to run):

pyproject.toml

\[project\]
name \= "mypackage"
version \= "0.0.1"
dependencies \= \[
    "requests",
    'importlib-metadata; python\_version<"3.10"',
\]

See [Configuring setuptools using pyproject.toml files](pyproject_config.html) for more information.

setup.cfg

\[metadata\]
name \= mypackage
version \= 0.0.1

\[options\]
install\_requires \=
    requests
    importlib-metadata; python\_version<"3.10"

See [Configuring setuptools using setup.cfg files](declarative_config.html) for more information.

setup.py [\[1\]](#setup-py)

from setuptools import setup

setup(
    name\='mypackage',
    version\='0.0.1',
    install\_requires\=\[
        'requests',
        'importlib-metadata; python\_version<"3.10"',
    \],
)

See [Keywords](../references/keywords.html) for more information.

Finally, you will need to organize your Python code to make it ready for distributing into something that looks like the following (optional files marked with `#`):

mypackage
├── pyproject.toml  # and/or setup.cfg/setup.py (depending on the configuration method)
|   # README.rst or README.md (a nice description of your package)
|   # LICENCE (properly chosen license information, e.g. MIT, BSD-3, GPL-3, MPL-2, etc...)
└── mypackage
    ├── \_\_init\_\_.py
    └── ... (other Python files)

With [build installed in your system](#install-build), you can then run:

python \-m build

You now have your distribution ready (e.g. a `tar.gz` file and a `.whl` file in the `dist` directory), which you can [upload](https://twine.readthedocs.io/en/stable/index.html "(in twine v5.1)") to [PyPI](https://pypi.org)!

Of course, before you release your project to [PyPI](https://pypi.org), you’ll want to add a bit more information to help people find or learn about your project. And maybe your project will have grown by then to include a few dependencies, and perhaps some data files and scripts. In the next few sections, we will walk through the additional but essential information you need to specify to properly package your project.

Info: Using `setup.py`

Setuptools offers first class support for `setup.py` files as a configuration mechanism.

It is important to remember, however, that running this file as a script (e.g. `python setup.py sdist`) is strongly **discouraged**, and that the majority of the command line interfaces are (or will be) **deprecated** (e.g. `python setup.py install`, `python setup.py bdist_wininst`, …).

We also recommend users to expose as much as possible configuration in a more _declarative_ way via the [pyproject.toml](pyproject_config.html) or [setup.cfg](declarative_config.html), and keep the `setup.py` minimal with only the dynamic parts (or even omit it completely if applicable).

See [Why you shouldn’t invoke setup.py directly](https://blog.ganssle.io/articles/2021/10/setup-py-deprecated.html) for more background.

Overview[¶](#overview "Link to this heading")
---------------------------------------------

### Package discovery[¶](#package-discovery "Link to this heading")

For projects that follow a simple directory structure, `setuptools` should be able to automatically detect all [packages](https://docs.python.org/3.11/glossary.html#term-package "(in Python v3.11)") and [namespaces](https://docs.python.org/3.11/glossary.html#term-namespace "(in Python v3.11)"). However, complex projects might include additional folders and supporting files that not necessarily should be distributed (or that can confuse `setuptools` auto discovery algorithm).

Therefore, `setuptools` provides a convenient way to customize which packages should be distributed and in which directory they should be found, as shown in the example below:

pyproject.toml

\# ...
\[tool.setuptools.packages\]
find \= {}  \# Scan the project directory with the default parameters

\# OR
\[tool.setuptools.packages.find\]
\# All the following settings are optional:
where \= \["src"\]  \# \["."\] by default
include \= \["mypackage\*"\]  \# \["\*"\] by default
exclude \= \["mypackage.tests\*"\]  \# empty by default
namespaces \= false  \# true by default

setup.cfg

\[options\]
packages \= find: \# OR \`find\_namespace:\` if you want to use namespaces

\[options.packages.find\]  # (always \`find\` even if \`find\_namespace:\` was used before)
\# This section is optional as well as each of the following options:
where\=src  \# . by default
include\=mypackage\*  \# \* by default
exclude\=mypackage.tests\*  \# empty by default

setup.py [\[1\]](#setup-py)

from setuptools import setup, find\_packages  \# or find\_namespace\_packages

setup(
    \# ...
    packages\=find\_packages(
        \# All keyword arguments below are optional:
        where\='src',  \# '.' by default
        include\=\['mypackage\*'\],  \# \['\*'\] by default
        exclude\=\['mypackage.tests'\],  \# empty by default
    ),
    \# ...
)

When you pass the above information, alongside other necessary information, `setuptools` walks through the directory specified in `where` (defaults to `.`) and filters the packages it can find following the `include` patterns (defaults to `*`), then it removes those that match the `exclude` patterns (defaults to empty) and returns a list of Python packages.

For more details and advanced use, go to [Package Discovery and Namespace Packages](package_discovery.html#package-discovery).

Tip

Starting with version 61.0.0, setuptools’ automatic discovery capabilities have been improved to detect popular project layouts (such as the [flat-layout](package_discovery.html#flat-layout) and [src-layout](package_discovery.html#src-layout)) without requiring any special configuration. Check out our [reference docs](package_discovery.html#package-discovery) for more information.

### Entry points and automatic script creation[¶](#entry-points-and-automatic-script-creation "Link to this heading")

Setuptools supports automatic creation of scripts upon installation, that run code within your package if you specify them as [entry points](https://packaging.python.org/en/latest/specifications/entry-points/ "(in Python Packaging User Guide)"). An example of how this feature can be used in `pip`: it allows you to run commands like `pip install` instead of having to type `python -m pip install`.

The following configuration examples show how to accomplish this:

pyproject.toml

\[project.scripts\]
cli-name \= "mypkg.mymodule:some\_func"

setup.cfg

\[options.entry\_points\]
console\_scripts \=
    cli-name \= mypkg.mymodule:some\_func

setup.py [\[1\]](#setup-py)

setup(
    \# ...
    entry\_points\={
        'console\_scripts': \[
            'cli-name = mypkg.mymodule:some\_func',
        \]
    }
)

When this project is installed, a `cli-name` executable will be created. `cli-name` will invoke the function `some_func` in the `mypkg/mymodule.py` file when called by the user. Note that you can also use the `entry-points` mechanism to advertise components between installed packages and implement plugin systems. For detailed usage, go to [Entry Points](entry_point.html).

### Dependency management[¶](#dependency-management "Link to this heading")

Packages built with `setuptools` can specify dependencies to be automatically installed when the package itself is installed. The example below shows how to configure this kind of dependencies:

pyproject.toml

\[project\]
\# ...
dependencies \= \[
    "docutils",
    "requests <= 0.4",
\]
\# ...

setup.cfg

\[options\]
install\_requires \=
    docutils
    requests <\= 0.4

setup.py [\[1\]](#setup-py)

setup(
    \# ...
    install\_requires\=\["docutils", "requests <= 0.4"\],
    \# ...
)

Each dependency is represented by a string that can optionally contain version requirements (e.g. one of the operators <, >, <=, >=, == or !=, followed by a version identifier), and/or conditional environment markers, e.g. `sys_platform == "win32"` (see [Version specifiers](https://packaging.python.org/en/latest/specifications/version-specifiers/ "(in Python Packaging User Guide)") for more information).

When your project is installed, all of the dependencies not already installed will be located (via PyPI), downloaded, built (if necessary), and installed. This, of course, is a simplified scenario. You can also specify groups of extra dependencies that are not strictly required by your package to work, but that will provide additional functionalities. For more advanced use, see [Dependencies Management in Setuptools](dependency_management.html).

### Including Data Files[¶](#including-data-files "Link to this heading")

Setuptools offers three ways to specify data files to be included in your packages. For the simplest use, you can simply use the `include_package_data` keyword:

pyproject.toml

\[tool.setuptools\]
include-package-data \= true
\# This is already the default behaviour if you are using
\# pyproject.toml to configure your build.
\# You can deactivate that with \`include-package-data = false\`

setup.cfg

\[options\]
include\_package\_data \= True

setup.py [\[1\]](#setup-py)

setup(
    \# ...
    include\_package\_data\=True,
    \# ...
)

This tells setuptools to install any data files it finds in your packages. The data files must be specified via the [MANIFEST.in](miscellaneous.html#using-manifest-in) file or automatically added by a [Revision Control System plugin](extension.html#adding-support-for-revision-control-systems). For more details, see [Data Files Support](datafiles.html).

### Development mode[¶](#development-mode "Link to this heading")

`setuptools` allows you to install a package without copying any files to your interpreter directory (e.g. the `site-packages` directory). This allows you to modify your source code and have the changes take effect without you having to rebuild and reinstall. Here’s how to do it:

pip install \--editable .

See [Development Mode (a.k.a. “Editable Installs”)](development_mode.html) for more information.

Tip

Prior to [pip v21.1](https://pip.pypa.io/en/latest/news/#v21-1 "(in pip v25.0)"), a `setup.py` script was required to be compatible with development mode. With late versions of pip, projects without `setup.py` may be installed in this mode.

If you have a version of `pip` older than v21.1 or is using a different packaging-related tool that does not support [**PEP 660**](https://peps.python.org/pep-0660/), you might need to keep a `setup.py` file in your repository if you want to use editable installs.

A simple script will suffice, for example:

from setuptools import setup

setup()

You can still keep all the configuration in [pyproject.toml](pyproject_config.html) and/or [setup.cfg](declarative_config.html)

Note

When building from source code (for example, by `python -m build` or `pip install -e .`) some directories hosting build artefacts and cache files may be created, such as `build`, `dist`, `*.egg-info` [\[2\]](#cache). You can configure your version control system to ignore them (see [GitHub’s .gitignore template](https://github.com/github/gitignore/blob/main/Python.gitignore) for an example).

### Uploading your package to PyPI[¶](#uploading-your-package-to-pypi "Link to this heading")

After generating the distribution files, the next step would be to upload your distribution so others can use it. This functionality is provided by [twine](https://pypi.org/project/twine) and is documented in the [Python packaging tutorial](https://packaging.python.org/en/latest/tutorials/packaging-projects/ "(in Python Packaging User Guide)").

### Transitioning from `setup.py` to declarative config[¶](#transitioning-from-setup-py-to-declarative-config "Link to this heading")

To avoid executing arbitrary scripts and boilerplate code, we are transitioning from defining all your package information by running `setup()` to doing this declaratively - by using `pyproject.toml` (or older `setup.cfg`).

To ease the challenges of transitioning, we provide a quick [guide](pyproject_config.html) to understanding how `pyproject.toml` is parsed by `setuptools`. (Alternatively, here is the [guide](declarative_config.html) for `setup.cfg`).

Note

The approach `setuptools` would like to take is to eventually use a single declarative format (`pyproject.toml`) instead of maintaining 2 (`pyproject.toml` / `setup.cfg`). Yet, chances are, `setup.cfg` will continue to be maintained for a long time.

Resources on Python packaging[¶](#resources-on-python-packaging "Link to this heading")
---------------------------------------------------------------------------------------

Packaging in Python can be hard and is constantly evolving. [Python Packaging User Guide](https://packaging.python.org) has tutorials and up-to-date references that can help you when it is time to distribute your work.

* * *

Notes

\[1\] ([1](#id2),[2](#id3),[3](#id4),[4](#id5),[5](#id6),[6](#id8))

New projects are advised to avoid `setup.py` configurations (beyond the minimal stub) when custom scripting during the build is not necessary. Examples are kept in this document to help people interested in maintaining or contributing to existing packages that use `setup.py`. Note that you can still keep most of configuration declarative in [setup.cfg](declarative_config.html) or [pyproject.toml](pyproject_config.html) and use `setup.py` only for the parts not supported in those files (e.g. C extensions). See [note](#setuppy-discouraged).

\[[2](#id9)\]

If you feel that caching is causing problems to your build, specially after changes in the configuration files, consider removing `build`, `dist`, `*.egg-info` before rebuilding or installing your project.

[

Next

Package Discovery and Namespace Packages

](package_discovery.html)[

Previous

Building and Distributing Packages with Setuptools



](index.html)

Copyright ©

Made with [Sphinx](https://www.sphinx-doc.org/) and [@pradyunsg](https://pradyunsg.me)'s [Furo](https://github.com/pradyunsg/furo)

On this page

*   [Quickstart](#)
    *   [Installation](#installation)
    *   [Basic Use](#basic-use)
    *   [Overview](#overview)
        *   [Package discovery](#package-discovery)
        *   [Entry points and automatic script creation](#entry-points-and-automatic-script-creation)
        *   [Dependency management](#dependency-management)
        *   [Including Data Files](#including-data-files)
        *   [Development mode](#development-mode)
        *   [Uploading your package to PyPI](#uploading-your-package-to-pypi)
        *   [Transitioning from `setup.py` to declarative config](#transitioning-from-setup-py-to-declarative-config)
    *   [Resources on Python packaging](#resources-on-python-packaging)