from setuptools import setup, find_packages

setup(name='barcos',
      version="0.0.1",
      description='Encuentra el barco que llega primero',
      author='David Pineda Osorio',
      author_email='dpineda@csn.uchile.cl',
      license='GPL3',
      install_requires=[
          "rich",
          "numpy"
      ],
      packages=find_packages(),
      include_package_data=True,      
      package_dir={'barcos': 'barcos'},
      zip_safe=False)
