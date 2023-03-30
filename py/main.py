#!/bin/python3

import cv2 as cv
import utility
import os

import cairosvg


prog = utility.Prog()


@prog.stage('input')
def imread():
    global img
    img = cv.imread(prog.config['input'])
    assert not img is None, "cannot read input" + prog.config['input']
    prog.log_picture(img, 'input')


@prog.stage('canny')
def canny():
    global img_canny
    global img_canny_path
    config_canny = prog.config['options_pre']['canny']
    img_canny = cv.Canny(
        img, config_canny['threshold1'], config_canny['threshold2'])
    assert not img_canny is None, "error with canny"
    img_canny_path = prog.path_jpg(prog.dir_inter, 'canny')
    cv.imwrite(img_canny_path, img_canny)
    prog.log_picture(img_canny, 'canny')

# cv.FastFeatureDetector_create()


@prog.stage('core')
def core():
    command_core = "cargo run --release -- "
    args = "--origin {origin} --input {input} --intermediate {inter} --log-directory {log}"
    args = args.format(origin=prog.config['input'],
                       input=img_canny_path,
                       inter=prog.dir_inter, log=prog.dir_log)
    options = " -i -t -T"
    prog.command_run('core', command_core + args + options)


@prog.stage('svg2png')
def svg2png():
    for p in os.listdir(prog.dir_log):
        name, ext = os.path.splitext(p)
        if ext != ".svg":
            continue
        input = prog.dir_log + '/' + p
        output = prog.dir_log + '/' + name + ".png"
        cairosvg.svg2png(url=input, write_to=output)


imread()
canny()
core()
svg2png()

exit(0)
