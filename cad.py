#!/usr/bin/env python
# coding: utf-8

# © Copyright 2021 Ivan Huamani [ihuamanis@uni.pe]
# License: GPL <http://www.gnu.org/copyleft/gpl.html>
# Source code: <https://github.com/ivansaul/gmi>

# # **Python CAD**

# ### **Packages**

# In[1]:


import os
import ezdxf
import pandas as pd
from ezdxf import zoom


# Draw function

def draw_dxf(csv_file):

    # ### **Import data**

    # In[2]:

    df=pd.read_csv(csv_file, usecols=[0,1,2,3,4], names=["est", "x", "y", "z", "des"], header=None)
    df.dropna(inplace=True)
    df.drop_duplicates(inplace=True)

    df['des']=df['des'].str.upper()
    df['xy']=df.apply(lambda j: (j['x'],j['y']),axis=1)
    df['xyz']=df.apply(lambda j: (j['x'],j['y'],j['z']),axis=1)


    # ### **Config DXF file**

    # In[3]:


    # Argument setup=True for adding standard linetypes and text styles.
    doc=ezdxf.new(dxfversion='R2000', setup=True)
    #doc=ezdxf.new(dxfversion='R12', setup=True)
    doc.header['$PDMODE'] = 35
    doc.header['$PDSIZE'] = 0.5

    #Layes dxfattribs
    layers_dxfattribs = {
        'L' :{'color':1},
        'T' :{'color':6},
        'P' :{'color':2},
        'L1':{'color':4},
        'L2':{'color':3},
        'L3':{'color':5},
        'L4':{'color':40},
        'SEC':{'color':7},
        'PP':{'color':188},
        '2D':{'color':7, 'linetype':'DASHED','lineweight':0.1}

    }

    msp=doc.modelspace()


    # ### **Sections**

    # In[4]:


    #Select only sections
    filt = (df['des']=='SEC')
    df_sec= df[filt][['est','xyz','des']]

    #find start : end of sections
    est_list = df_sec['est'].tolist()
    est_list = [int(x) for x in est_list]

    idx=[]
    for i,v in enumerate(est_list):
        if v==1:
            idx=idx+[i]

    #print(idx)

    #Build layer 'SEC'
    if len(idx)>0:
        doc.layers.new(name='SEC', dxfattribs=layers_dxfattribs['SEC'])

    #Draw sections
    for i in range(len(idx)):
        if i < len(idx)-1:
            points = df_sec['xyz'][idx[i]:idx[i+1]].tolist()
            msp.add_polyline3d(points=points, close=True, dxfattribs={'layer':'SEC'})
            
        else:
            points = df_sec['xyz'][idx[i]:].tolist()
            msp.add_polyline3d(points=points, close=True, dxfattribs={'layer':'SEC'})


    # ### **LTP**

    # In[5]:


    #Select only LTP
    cases = ['L','L1','L2','L3','L4','T','P']
    filt = df['des'].isin(cases)
    df_ltp = df.loc[filt,['des','xyz','xy']]
    df_ltp.reset_index(drop=True, inplace=True)

    #find start : end of LTP
    ltp_list = df_ltp['des'].tolist()

    idx=[]
    jo = 7777
    vo = 'any'

    for i,v in enumerate(ltp_list):
        if v!=vo:
            idx=idx+[i]
            jo=i
            vo=v

    #Build layers
    for l in df_ltp['des'].unique():
        doc.layers.new(name=l, dxfattribs=layers_dxfattribs[l])
        if l=='L':
            doc.layers.new(name='2D', dxfattribs=layers_dxfattribs['2D'])

    #Draw LTP
    for i in range(len(idx)):
        if i < len(idx)-1:
            points = df_ltp['xyz'][idx[i]:idx[i+1]].tolist()
            msp.add_polyline3d(points=points, close=False, dxfattribs={'layer':df_ltp['des'][idx[i]]})
            if df_ltp['des'][idx[i]]=='L':
                points_2d = df_ltp['xy'][idx[i]:idx[i+1]].tolist()
                msp.add_lwpolyline(points=points_2d,dxfattribs={'layer':'2D'})
            
        else:
            points = df_ltp['xyz'][idx[i]:].tolist()
            msp.add_polyline3d(points=points, close=False, dxfattribs={'layer':df_ltp['des'][idx[i]]})
            if df_ltp['des'][idx[i]]=='L':
                points_2d = df_ltp['xy'][idx[i]:].tolist()
                msp.add_lwpolyline(points=points_2d,dxfattribs={'layer':'2D'})


    # ### **3D Points**

    # In[6]:


    #Select only points
    filt = (df['des']=='PP')
    df_pp= df[filt][['est','xyz','xy']]
    pps = df_pp.values.tolist()

    #Build layer 'PP'
    if len(pps)>0:
        doc.layers.new(name='PP', dxfattribs=layers_dxfattribs['PP'])

    #Draw 3D points
    for pp in pps:
        msp.add_point(pp[1], dxfattribs={'layer':'PP'})
        msp.add_text(pp[0],
                    dxfattribs={
                        'layer':'PP',
                        'style': 'LiberationSerif',
                        'height': 0.5}
                    ).set_pos(pp[1], align='LEFT')

    #Draw 2D Points
    if 'PP' in doc.layers:
        for pp in pps:
            msp.add_point(pp[2], dxfattribs={'layer':'2D'})
            msp.add_text(pp[0],
                        dxfattribs={
                            'layer':'2D',
                            'style': 'LiberationSerif',
                            'height': 0.5}
                        ).set_pos(pp[2], align='LEFT')


    # ### **Switch 2D layer off**

    # In[7]:


    for layer in doc.layers:
        if layer.dxf.name == '2D':
            layer.off()


    # ### **Save as dxf file**

    # In[8]:


    zoom.extents(msp)
    doc.saveas(f'{csv_file[:-4]}.dxf')


# Inplementation

files = os.listdir()
csv_files = []

for file in files:
    if file.endswith('.CSV') or file.endswith('.csv'):
        csv_files.append(file)


for i, csv_file in enumerate(csv_files):
    draw_dxf(csv_file)
    print(f'[{i+1}/{len(csv_files)}] {csv_file[:-4]}.dxf file was built successfully')

print('\n© Copyright 2021 ivansaul [ihuamanis@uni.pe]\n')
