#!/usr/bin/python

# A simple script to get a vim configuration
# from a git repository

import os
import re
import sys

import subprocess
import shutil
import urllib
import zipfile
import ConfigParser
import StringIO

VIMCONF_DIR = "~/.local/share/vimconf"
VIMCONF_DIR = os.path.expanduser(VIMCONF_DIR)
VIMCONF_DIR = os.path.normpath(VIMCONF_DIR)

VIMRC_TEMPLATE = """
" Auto-generated code. Do not edit

source {pathogen_autoload}
call pathogen#infect("{vimconf_dir}")
source {vimrc}

"""

def rm_rf(dest):
    """ Contrary to shutil.rm_rf,
    this wont't fail if dest does not
    exist and won't fail while trying to remove
    read-only files

    """
    def _rmtree_handler(func, path, _execinfo):
        """Call by rmtree when there was an error.

        If this is called because we could not remove a file, then see if
        it is readonly, change it back to nornal and try again
        """
        import stat
        if (func == os.remove) and not os.access(path, os.W_OK):
            os.chmod(path, stat.S_IWRITE)
            os.remove(path)
        else:
            # Something else must be wrong...
            raise
    if not os.path.exists(dest):
        return
    shutil.rmtree(dest, False, _rmtree_handler)

def mkdir_p(dest_dir):
    """ Contrary to os.makedirs, this
    wont't fail if dest already exists

    """
    if os.path.exists(dest_dir):
        return
    os.makedirs(dest_dir)


def get_backup_name(filename):
  id = 0
  while True:
    backup = "%s.%d" % (filename, id)
    id += 1
    if not os.path.exists(backup):
      return backup

def is_git(url):
    """ Check if an url is a git url

    """
    # This should do it:
    if url.startswith("git://"):
        return True
    if url.endswith(".git"):
        return True
    return False

def is_vimorg(url):
    """ Check if an url is a url from vimorg

    """
    if re.match('http://www.vim.org/scripts/download_script.php\?src_id=\d+', url):
        return True
    return False

def get_from_git(name, url):
    """ Fetch something from a git url

    """
    dest = os.path.join(VIMCONF_DIR, name)
    if not os.path.exists(dest):
        cmd = ["git", "clone", url, dest]
        subprocess.check_call(cmd, stdout=subprocess.PIPE)
    else:
        cmd = ["git", "fetch"]
        subprocess.check_call(cmd, cwd=dest, stdout=subprocess.PIPE)
        cmd = ["git", "reset", "--hard", "origin/master"]
        subprocess.check_call(cmd, cwd=dest, stdout=subprocess.PIPE)

def get_vim_script(name, contents):
    """ Install a script given its contents

    """
    dest = os.path.join(VIMCONF_DIR, name)
    plugin_dir = os.path.join(dest, "plugin")
    mkdir_p(plugin_dir)
    dest = os.path.join(plugin_dir, name + ".vim")
    with open(dest, "w") as fp:
        fp.write(contents)

def get_vim_zip(name, contents):
    """ Install a plugin where contents is
    the raw data of the zip

    """
    fp = StringIO.StringIO(contents)
    archive = zipfile.ZipFile(fp)
    dest = os.path.join(VIMCONF_DIR, name)
    rm_rf(dest)
    mkdir_p(dest)
    for member in archive.namelist():
        if member.endswith("/"):
            continue
        archive.extract(member, path=dest)

def get_vim_vba(name, contents):
    """ Install a plugin given the raw
    data of the .vba

    """
    lines = contents.splitlines(True)
    line_no = 3
    to_write = dict()
    file_contents = ""
    while(line_no < len(lines)):
        file_name = lines[line_no].strip()
        size = int(lines[line_no+1])
        file_contents = lines[line_no+2:line_no+2+size]
        line_no  += size+2
        to_write[file_name] = file_contents

    dest = os.path.join(VIMCONF_DIR, name)
    rm_rf(dest)
    mkdir_p(dest)
    for (file_name, file_contents) in to_write.iteritems():
        full_filename = os.path.join(dest, file_name)
        mkdir_p(os.path.dirname(full_filename))
        with open(full_filename, "w") as fp:
            fp.writelines(file_contents)

def get_from_vimorg(name, url):
    """ Vim.org uses php to get download links with a
    Content-Disposition header

    """
    url_obj = urllib.urlopen(url)
    data = url_obj.read()
    url_obj.close()
    content = url_obj.headers.getheader("Content-Disposition")
    attached_file = content.split("=")[-1]
    if attached_file.endswith(".vim"):
        get_vim_script(name, data)
    elif attached_file.endswith(".zip"):
        get_vim_zip(name, data)
    elif attached_file.endswith(".vba"):
        get_vim_vba(name, data)


def backup_conf():
    """ Backup vim configuration

    """
    vimrc = os.path.expanduser("~/.vimrc")
    if os.path.exists(vimrc):
        backup = get_backup_name(vimrc)
        os.rename(vimrc, backup)
        print "~/.vimrc backuped to", backup

def get_plugins(cfg_path):
    """ Install plugins, where plugins in a list
    of (name, url)

    """
    parser = ConfigParser.RawConfigParser()
    parser.read(cfg_path)
    plugins = parser.items("plugins")
    for (name, url) in plugins:
        print "Adding plugin %s ..." % name
        if is_git(url):
            get_from_git(name, url)
        elif is_vimorg(url):
            get_from_vimorg(name, url)

def build_plugins(cfg_path):
    """ Build the plugins that need to be built

    """
    parser = ConfigParser.RawConfigParser()
    parser.read(cfg_path)
    to_build = parser.items("build")
    for (name, command) in to_build:
        print "Building ", name, "..."
        plugin_path = os.path.join(VIMCONF_DIR, name)
        if command == "rake":
            try:
                subprocess.check_call(["rake", "make"],
                    cwd=plugin_path)
            except Exception, e:
                print "Could not build", name
                print "Error was: ", e

def install_vim_conf(vim_conf_url):
    """ Install vimconf:

    - backup existing config
    - create custom vimrc
    """
    backup_conf()
    pathogen_autoload = os.path.join(VIMCONF_DIR,
      "pathogen/autoload/pathogen.vim")
    get_from_git("vimconf", vim_conf_url)
    vimrc = os.path.join(VIMCONF_DIR, "vimconf/vimrc")
    to_write = VIMRC_TEMPLATE.format(
      vimconf_dir=VIMCONF_DIR,
      pathogen_autoload=pathogen_autoload,
      vimrc=vimrc)
    dest = os.path.expanduser("~/.vimrc")
    with open(dest, "w") as fp:
      fp.write(to_write)


def main():
    """ Main entry point

    """
    # Install this repository as a bundle
    mkdir_p(VIMCONF_DIR)
    vimconf = os.path.join(VIMCONF_DIR, "vimconf")
    rm_rf(vimconf)
    this_dir = os.path.dirname(__file__)
    src_dir  = os.path.join(this_dir, "..")
    shutil.copytree(src_dir, vimconf)
    backup_conf()
    pathogen_autoload = os.path.join(VIMCONF_DIR,
      "pathogen/autoload/pathogen.vim")
    vimrc = os.path.join(VIMCONF_DIR, "vimconf/vimrc")
    to_write = VIMRC_TEMPLATE.format(
      vimconf_dir=VIMCONF_DIR,
      pathogen_autoload=pathogen_autoload,
      vimrc=vimrc)
    dest = os.path.expanduser("~/.vimrc")
    with open(dest, "w") as fp:
      fp.write(to_write)
    vimconf_cfg = os.path.join(vimconf, "vimconf.cfg")
    get_plugins(vimconf_cfg)
    # some plugins need additional build steps
    build_plugins(vimconf_cfg)

if __name__ == "__main__":
    main()
